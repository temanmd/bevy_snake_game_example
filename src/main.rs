use bevy::prelude::*;

mod config;
mod movement;
mod prize;
mod setup;
mod snake;

use movement::MovementPlugin;
use prize::PrizePlugin;
use setup::SetupPlugin;
use snake::SnakePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(PrizePlugin)
        .run();
}
