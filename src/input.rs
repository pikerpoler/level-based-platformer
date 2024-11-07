use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::player::Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Up,
    Down,
    Left,
    Right,
    Jump,
}

pub struct InputPlugin;

fn setup_controls(mut commands: Commands) {
    let mut input_map = InputMap::default();
    input_map.insert_multiple([
        (PlayerAction::Left, KeyCode::KeyA),
        (PlayerAction::Left, KeyCode::ArrowLeft),
        (PlayerAction::Right, KeyCode::KeyD),
        (PlayerAction::Right, KeyCode::ArrowRight),
        (PlayerAction::Up, KeyCode::KeyW),
        (PlayerAction::Up, KeyCode::ArrowUp),
        (PlayerAction::Down, KeyCode::KeyS),
        (PlayerAction::Down, KeyCode::ArrowDown),
        (PlayerAction::Jump, KeyCode::Space),
    ]);
    input_map.insert_multiple([
        (PlayerAction::Left, GamepadButtonType::DPadLeft),
        (PlayerAction::Right, GamepadButtonType::DPadRight),
        (PlayerAction::Up, GamepadButtonType::DPadUp),
        (PlayerAction::Down, GamepadButtonType::DPadDown),
        (PlayerAction::Jump, GamepadButtonType::South),
    ]);
    input_map.insert_dual_axis(PlayerAction::Move, GamepadStick::LEFT);
    commands
        .spawn(InputManagerBundle::with_map(input_map))
        .insert(Player);
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, setup_controls);
    }
}
