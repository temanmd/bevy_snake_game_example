use crate::{
    config::PRIZE_COLOR,
    setup::{Game, MoveTimer},
};
use bevy::prelude::*;
use rand::prelude::*;

pub struct PrizePlugin;

impl Plugin for PrizePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_prize);
    }
}

fn spawn_prize(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    timer: Res<MoveTimer>,
) {
    if game.prize.entity.is_some() {
        return;
    }
    if timer.0.finished() {
        let mut rng = rand::rng();

        let window_width = 1280.0 / 2.0;
        let window_height = 720.0 / 2.0;
        let block_width = 20.0;
        let horizontal_blocks_count = window_width / block_width;
        let vertical_blocks_count = window_height / block_width;
        let min = 0;
        let x_rnd_max_position = horizontal_blocks_count - 1.0;
        let y_rnd_max_position = vertical_blocks_count - 1.0;
        let x_rnd_position = rng.random_range(min..=x_rnd_max_position as i32) as f32;
        let y_rnd_position = rng.random_range(min..=y_rnd_max_position as i32) as f32;
        let mut x_rnd = x_rnd_position * block_width;
        let mut y_rnd = y_rnd_position * block_width;

        if rng.random_bool(0.5) {
            x_rnd = -x_rnd;
        }
        if rng.random_bool(0.5) {
            y_rnd = -y_rnd;
        }

        game.prize.entity = Some(
            commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(PRIZE_COLOR)),
                    Transform::from_xyz(x_rnd, y_rnd, 1.0).with_scale(Vec3::splat(20.)),
                ))
                .id(),
        );
    }
}
