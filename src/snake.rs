use crate::{config::*, setup::*};
use bevy::prelude::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, update_snake);
    }
}

fn update_snake(
    time: Res<Time>,
    mut commands: Commands,
    mut timer: ResMut<MoveTimer>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    material_query: Query<&MeshMaterial2d<ColorMaterial>>,
) {
    if timer.0.tick(time.delta()).finished() {
        let transform = transforms
            .get_mut(game.parts.front().unwrap().entity.unwrap())
            .unwrap();
        let head_translation = transform.translation;

        let new_head_transform = match game.direction {
            Direction::Up => Transform::from_xyz(head_translation.x, head_translation.y + 20., 1.),
            Direction::Down => {
                Transform::from_xyz(head_translation.x, head_translation.y - 20., 0.)
            }
            Direction::Left => {
                Transform::from_xyz(head_translation.x - 20., head_translation.y, 0.)
            }
            Direction::Right => {
                Transform::from_xyz(head_translation.x + 20., head_translation.y, 0.)
            }
        };

        let old_head_entity = game.parts.front().unwrap().entity.unwrap();
        let old_head_material_id = material_query.get(old_head_entity).unwrap();

        let old_head_material = materials.get_mut(old_head_material_id).unwrap();
        old_head_material.color = SNAKE_COLOR;

        game.parts.push_front(Part {
            entity: Some(
                commands
                    .spawn((
                        Mesh2d(meshes.add(Rectangle::default())),
                        MeshMaterial2d(materials.add(SNAKE_HEAD_COLOR)),
                        new_head_transform.with_scale(Vec3::splat(20.)),
                    ))
                    .id(),
            ),
        });

        let prize_transform = transforms.get_mut(game.prize.entity.unwrap()).unwrap();

        if prize_transform.translation.x == new_head_transform.translation.x
            && prize_transform.translation.y == new_head_transform.translation.y
        {
            commands
                .entity(game.prize.entity.unwrap())
                .despawn_recursive();

            game.prize.entity = None;
        } else if game.prize.entity.is_some() {
            commands
                .entity(game.parts.back().unwrap().entity.unwrap())
                .despawn_recursive();

            game.parts.pop_back().unwrap();
        }

        game.direction_changed = false;
    }
}
