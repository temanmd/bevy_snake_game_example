use crate::setup::*;
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, movement_system);
    }
}

fn movement_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    let inputs = vec![KeyCode::KeyA, KeyCode::KeyD, KeyCode::KeyW, KeyCode::KeyS];
    if game.direction_changed || !keyboard_input.any_pressed(inputs) {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) && game.direction != Direction::Right {
        game.direction = Direction::Left;
        game.direction_changed = true;
    } else if keyboard_input.just_pressed(KeyCode::KeyD) && game.direction != Direction::Left {
        game.direction = Direction::Right;
        game.direction_changed = true;
    } else if keyboard_input.just_pressed(KeyCode::KeyW) && game.direction != Direction::Down {
        game.direction = Direction::Up;
        game.direction_changed = true;
    } else if keyboard_input.just_pressed(KeyCode::KeyS) && game.direction != Direction::Up {
        game.direction = Direction::Down;
        game.direction_changed = true;
    }
}
