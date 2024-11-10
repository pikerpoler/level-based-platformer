use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::audio::AudioEvent;
use crate::climbing::Climber;
use crate::constants::GAMEPAD_SENSITIVITY_THRESHOLD;
use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};

use crate::input::PlayerAction;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("images/ghost.png")]
    pub sprite_bundle: SpriteBundle,
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
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection), With<Player>>,
    mut audio_event: EventWriter<AudioEvent>,
) {
    let mut input = input.single_mut().to_owned();
    handle_axis_movement(&mut input);

    for (mut velocity, mut climber, ground_detection) in &mut query {
        let right = if input.pressed(&PlayerAction::Right) {
            1.
        } else {
            0.
        };
        let left = if input.pressed(&PlayerAction::Left) {
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, player_movement);
    }
}
