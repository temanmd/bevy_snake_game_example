use crate::setup::{Direction, Game};
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, movement_system);
    }
}

fn movement_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    let inputs = vec![KeyCode::KeyA, KeyCode::KeyD, KeyCode::KeyW, KeyCode::KeyS];
    if game.direction_changed
        || !keyboard_input.any_pressed(inputs)
        || !is_valid_move(&game.direction, &keyboard_input)
    {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        game.direction = Direction::Left;
    } else if keyboard_input.just_pressed(KeyCode::KeyD) {
        game.direction = Direction::Right;
    } else if keyboard_input.just_pressed(KeyCode::KeyW) {
        game.direction = Direction::Up;
    } else if keyboard_input.just_pressed(KeyCode::KeyS) {
        game.direction = Direction::Down;
    }
    game.direction_changed = true;
}

fn is_valid_move(direction: &Direction, input: &ButtonInput<KeyCode>) -> bool {
    match direction {
        Direction::Left | Direction::Right => {
            if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::KeyS) {
                return true;
            }
        }
        Direction::Up | Direction::Down => {
            if input.just_pressed(KeyCode::KeyA) || input.just_pressed(KeyCode::KeyD) {
                return true;
            }
        }
    }

    false
}
