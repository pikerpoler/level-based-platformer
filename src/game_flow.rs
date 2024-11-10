use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::int_grid_index_to_grid_coords;
use bevy_rapier2d::prelude::*;

use crate::audio::AudioEvent;
use crate::colliders::SensorBundle;
use crate::constants::{IntGridValues, TILE_SIZE};
use crate::player::Player;

#[derive(Resource, Clone)]
pub struct GameState {
    pub current_level: usize,
}

impl GameState {
    pub fn current_level_identifier(&self) -> LevelSelection {
        LevelSelection::Identifier(format!("World_Level_{}", self.current_level))
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct SpwanPoint;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct SpawnPointBundle {
    pub spawn_point: SpwanPoint,
}
#[derive(Event)]
pub struct NextLevel;

#[derive(Event)]
pub struct RestartLevel;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_handle = asset_server.load("levels/all_levels.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
    let starting_game_state = GameState { current_level: 0 };
    commands.insert_resource(starting_game_state.clone());
    commands.insert_resource(starting_game_state.current_level_identifier());
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

fn set_camera_to_level_size(
    mut camera_query: Query<(&mut Transform, &OrthographicProjection), With<MainCamera>>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    asset_server: Res<AssetServer>,
    state: ResMut<GameState>,
) {
    let project_id = ldtk_projects.single();
    if asset_server.get_load_state(project_id) != Some(bevy::asset::LoadState::Loaded) {
        return;
    }

    let ldtk_project = ldtk_project_assets
        .get(project_id)
        .expect("Project should be loaded if level has spawned");

    let level = ldtk_project
        .find_raw_level_by_level_selection(&state.current_level_identifier())
        .expect("Spawned level should exist in LDtk project");

    let (mut transform, ortho) = camera_query.single_mut();
    // these are not in use, but good to know they exist
    // let worldx = level.world_x as f32;
    // let worldy = level.world_y as f32;
    let width = level.px_wid as f32;
    let height = level.px_hei as f32;
    let area = ortho.area;

    // figure our what dimension will limit our scale
    let level_ratio = width / height;
    let camera_ratio = (area.max.x - area.min.x) / (area.max.y - area.min.y);
    let scale = if level_ratio > camera_ratio {
        // width is limiting
        width / (area.max.x - area.min.x)
    } else {
        // height is limiting
        height / (area.max.y - area.min.y)
    };

    transform.translation.x = width / 2.0;
    transform.translation.y = height / 2.0;
    transform.scale.x = scale;
    transform.scale.y = scale;
}

fn restart_level(
    mut commands: Commands,
    mut restart_level_reader: EventReader<RestartLevel>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    asset_server: Res<AssetServer>,
    state: ResMut<GameState>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    for _restart_level in restart_level_reader.read() {
        let level_selection = state.current_level_identifier();
        commands.insert_resource(level_selection);
        // get_spawn_point_coordinates.

        let project_id = ldtk_projects.single();
        if asset_server.get_load_state(project_id) != Some(bevy::asset::LoadState::Loaded) {
            return;
        }

        let ldtk_project = ldtk_project_assets
            .get(project_id)
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .find_raw_level_by_level_selection(&state.current_level_identifier())
            .expect("Spawned level should exist in LDtk project");
        let level_instances = level
            .layer_instances
            .as_ref()
            .expect("failed to get level instances");

        let coords = extract_intgrid_coordinates(
            level_instances,
            &["Objective_Tiles".to_string()],
            &[IntGridValues::SpawnPoint], // 5 is the intgrid value for spawn point
        );

        assert!(
            coords.len() == 1,
            "Expected exactly 1 spawn point per level"
        );

        // set player position to spawn point
        let spawn_point = coords[0];
        if let Ok((mut transform, _player)) = player_query.get_single_mut() {
            transform.translation.x = (spawn_point.x * TILE_SIZE + TILE_SIZE / 2) as f32;
            transform.translation.y = (spawn_point.y * TILE_SIZE + TILE_SIZE / 2) as f32;
        }
    }
}
pub fn finish_level(
    players: Query<Entity, With<Player>>,
    goals: Query<Entity, With<Goal>>,
    mut collisions: EventReader<CollisionEvent>,
    mut next_level: EventWriter<NextLevel>,
    mut audio_event: EventWriter<AudioEvent>,
) {
    for collision in collisions.read() {
        if let CollisionEvent::Started(collider_a, collider_b, _) = collision {
            if (goals.get(*collider_a).is_ok() && players.get(*collider_b).is_ok())
                || (players.get(*collider_a).is_ok() && goals.get(*collider_b).is_ok())
            {
                next_level.send(NextLevel);
                audio_event.send(AudioEvent::LevelComplete);
            }
        }
    }
}

fn level_selection(
    mut state: ResMut<GameState>,
    input: Res<ButtonInput<KeyCode>>,
    mut next_level_reader: EventReader<NextLevel>,
    mut restart_level_writer: EventWriter<RestartLevel>,
) {
    if input.just_pressed(KeyCode::Digit0) {
        println!("level 0 selected");
        state.current_level = 0;
        restart_level_writer.send(RestartLevel);
    }
    if input.just_pressed(KeyCode::Digit1) {
        println!("level 1 selected");
        state.current_level = 1;
        restart_level_writer.send(RestartLevel);
    }
    if input.just_pressed(KeyCode::Digit2) {
        println!("level 2 selected");
        state.current_level = 2;
        restart_level_writer.send(RestartLevel);
    }
    for _event in next_level_reader.read() {
        state.current_level += 1;
        println!("level {} selected", state.current_level);
        restart_level_writer.send(RestartLevel);
    }
}

fn extract_intgrid_coordinates(
    layer_instances: &[LayerInstance],
    layer_names: &[String],
    values_of_interest: &[IntGridValues],
) -> Vec<GridCoords> {
    let mut grid_coords: Vec<GridCoords> = Vec::new();
    for layer_instance in layer_instances.iter() {
        let name = layer_instance.identifier.to_owned();
        if !layer_names.contains(&name) {
            continue;
        }
        for value in values_of_interest.iter() {
            // check where the value is in the layer.int_grid_csv
            let new_coords = layer_instance.int_grid_csv.iter().enumerate().filter(|(_, v)| **v == *value as i32).map(|(i, _)| {
                    int_grid_index_to_grid_coords(i, layer_instance.c_wid as u32,
                            layer_instance.c_hei as u32,).expect("int_grid_csv indices should be within the bounds of 0..(layer_width * layer_height)")
        }).collect::<Vec<GridCoords>>();
            grid_coords.extend(new_coords);
        }
    }
    grid_coords
}
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Goal;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct GoalBundle {
    #[from_int_grid_cell]
    pub sensor_bundle: SensorBundle,
    pub goal: Goal,
}

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, set_camera_to_level_size)
            .add_systems(Update, level_selection)
            .add_event::<NextLevel>()
            .add_systems(Update, restart_level)
            .add_event::<RestartLevel>()
            .add_systems(Update, finish_level)
            .register_ldtk_int_cell::<GoalBundle>(IntGridValues::Goal as i32);
    }
}
