use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod climbing;
mod colliders;
mod constants;
mod game_flow;
mod ground_detection;
mod input;
mod player;
mod walls;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(game_flow::GameFlowPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(climbing::ClimbingPlugin)
        .run();
}
