use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::audio::AudioEvent;
use crate::climbing::Climber;
use crate::constants::sprites::{
    fox::{
        CLIMB_FRAMES, CLIMB_FRAMES_IDLE, IDLE_FRAMES, JUMP_DOWN_FRAMES, JUMP_UP_FRAMES, WALK_FRAMES,
    },
    FrameRange,
};
use crate::constants::GAMEPAD_SENSITIVITY_THRESHOLD;
use crate::utils::is_almost_zero;
use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};

use crate::input::PlayerAction;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

impl AnimationIndices {
    pub fn set(&mut self, frame_range: FrameRange) {
        self.first = frame_range.first;
        self.last = frame_range.last;
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Component)]
struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub enum Facing {
    #[default]
    Right,
    Left,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player {
    pub facing: Facing,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    // #[sprite_bundle("images/ghost.png")]
    // pub sprite_bundle: SpriteBundle,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,

    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub climber: Climber,
    pub ground_detection: GroundDetection,
    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,

    pub animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
}

fn handle_axis_movement(input: &mut ActionState<PlayerAction>) {
    if input.axis_pair(&PlayerAction::Move) != Vec2::ZERO {
        let v = input.axis_pair(&PlayerAction::Move);

        // Horizontal movement
        match v.x {
            x if x < -GAMEPAD_SENSITIVITY_THRESHOLD => {
                input.press(&PlayerAction::Left);
                input.release(&PlayerAction::Right);
            }
            x if x > GAMEPAD_SENSITIVITY_THRESHOLD => {
                input.press(&PlayerAction::Right);
                input.release(&PlayerAction::Left);
            }
            _ => {
                if !input.pressed(&PlayerAction::Left) && !input.pressed(&PlayerAction::Right) {
                    input.release(&PlayerAction::Left);
                    input.release(&PlayerAction::Right);
                }
            }
        }

        // Vertical movement
        match v.y {
            y if y < -GAMEPAD_SENSITIVITY_THRESHOLD => {
                input.press(&PlayerAction::Down);
                input.release(&PlayerAction::Up);
            }
            y if y > GAMEPAD_SENSITIVITY_THRESHOLD => {
                input.press(&PlayerAction::Up);
                input.release(&PlayerAction::Down);
            }
            _ => {
                if !input.pressed(&PlayerAction::Up) && !input.pressed(&PlayerAction::Down) {
                    input.release(&PlayerAction::Up);
                    input.release(&PlayerAction::Down);
                }
            }
        }
    }
}

fn player_movement(
    mut input: Query<&mut ActionState<PlayerAction>, With<Player>>,
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection, &mut Player)>,
    mut audio_event: EventWriter<AudioEvent>,
) {
    let mut input = input.single_mut().to_owned();
    handle_axis_movement(&mut input);

    for (mut velocity, mut climber, ground_detection, mut player) in &mut query {
        let right = if input.pressed(&PlayerAction::Right) {
            player.facing = Facing::Right;
            1.
        } else {
            0.
        };
        let left = if input.pressed(&PlayerAction::Left) {
            player.facing = Facing::Left;
            1.
        } else {
            0.
        };

        velocity.linvel.x = (right - left) * 200.;

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(&PlayerAction::Up) || input.just_pressed(&PlayerAction::Down) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(&PlayerAction::Up) {
                1.
            } else {
                0.
            };
            let down = if input.pressed(&PlayerAction::Down) {
                1.
            } else {
                0.
            };

            velocity.linvel.y = (up - down) * 200.;
        }

        if input.just_pressed(&PlayerAction::Jump)
            && (ground_detection.on_ground || climber.climbing)
        {
            audio_event.send(AudioEvent::Jump);
            velocity.linvel.y = 400.;
            climber.climbing = false;
        }
    }
}

fn set_animation(
    mut query: Query<(
        &mut Velocity,
        &mut Climber,
        &GroundDetection,
        &mut Player,
        &mut AnimationIndices,
    )>,
) {
    for (velocity, climber, ground_detection, _player, mut animation_indices) in &mut query {
        let going_up = !ground_detection.on_ground && velocity.linvel.y > 0.;
        let is_falling =
            !ground_detection.on_ground && velocity.linvel.y <= 0. && !climber.climbing;
        let is_idle = is_almost_zero(velocity.linvel.x) && is_almost_zero(velocity.linvel.y);

        match (climber.climbing, going_up, is_falling, is_idle) {
            (true, _, _, true) => animation_indices.set(CLIMB_FRAMES_IDLE),
            (true, _, _, false) => animation_indices.set(CLIMB_FRAMES),
            (false, true, _, _) => animation_indices.set(JUMP_UP_FRAMES),
            (false, false, true, _) => animation_indices.set(JUMP_DOWN_FRAMES),
            (false, false, false, true) => animation_indices.set(IDLE_FRAMES),
            (false, false, false, false) => animation_indices.set(WALK_FRAMES),
        }
    }
}

fn animate_player(
    mut query: Query<(
        &Player,
        &mut TextureAtlas,
        &mut Sprite,
        &AnimationIndices,
        &mut AnimationTimer,
    )>,
    time: Res<Time>,
) {
    for (player, mut atlas, mut sprite, indices, mut timer) in &mut query {
        let timer = &mut timer.0;
        timer.tick(time.delta());

        match player.facing {
            Facing::Right => sprite.flip_x = false,
            Facing::Left => sprite.flip_x = true,
        }

        if timer.just_finished() {
            atlas.index = if atlas.index >= indices.last || atlas.index < indices.first {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, set_animation)
            .add_systems(Update, animate_player)
            .add_systems(Update, player_movement);
    }
}
