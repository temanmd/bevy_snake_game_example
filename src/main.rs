use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Default)]
struct Prize {
    entity: Option<Entity>,
}

#[derive(Default, Debug)]
struct Part {
    entity: Option<Entity>,
}

#[derive(Resource, Default)]
struct Game {
    prize: Prize,
    parts: VecDeque<Part>,
    direction: Direction,
}

#[derive(Default, PartialEq, Eq)]
enum Direction {
    Left,
    #[default]
    Right,
    Up,
    Down,
}

#[derive(Resource)]
struct MoveTimer(Timer);

struct HelloPlugin;

const SNAKE_COLOR: Color = Color::srgb(0.2, 1.0, 0.4);
const PRIZE_COLOR: Color = Color::srgb(1.0, 0.2, 0.4);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveTimer(Timer::from_seconds(0.2, TimerMode::Repeating)));
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, (movement_system, update_snake, spawn_prize));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game: ResMut<Game>,
) {
    commands.spawn(Camera2d);

    game.parts.push_front(Part {
        entity: Some(
            commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(SNAKE_COLOR)),
                    Transform::default().with_scale(Vec3::splat(20.)),
                ))
                .id(),
        ),
    });

    game.prize.entity = Some(
        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(PRIZE_COLOR)),
                Transform::from_xyz(0.0, 200.0, 0.0).with_scale(Vec3::splat(20.)),
            ))
            .id(),
    );

    commands.spawn((
        Text::new("Scores: 22"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn movement_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    if keyboard_input.pressed(KeyCode::KeyA) && game.direction != Direction::Right {
        game.direction = Direction::Left;
    } else if keyboard_input.pressed(KeyCode::KeyD) && game.direction != Direction::Left {
        game.direction = Direction::Right;
    } else if keyboard_input.pressed(KeyCode::KeyW) && game.direction != Direction::Down {
        game.direction = Direction::Up;
    } else if keyboard_input.pressed(KeyCode::KeyS) && game.direction != Direction::Up {
        game.direction = Direction::Down;
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
) {
    if timer.0.tick(time.delta()).finished() {
        let transform = transforms
            .get_mut(game.parts.front().unwrap().entity.unwrap())
            .unwrap();
        let head_translation = transform.translation;

        let new_head_transform = match game.direction {
            Direction::Up => Transform::from_xyz(head_translation.x, head_translation.y + 20., 0.0),
            Direction::Down => {
                Transform::from_xyz(head_translation.x, head_translation.y - 20., 0.0)
            }
            Direction::Left => {
                Transform::from_xyz(head_translation.x - 20., head_translation.y, 0.0)
            }
            Direction::Right => {
                Transform::from_xyz(head_translation.x + 20., head_translation.y, 0.0)
            }
        };

        game.parts.push_front(Part {
            entity: Some(
                commands
                    .spawn((
                        Mesh2d(meshes.add(Rectangle::default())),
                        MeshMaterial2d(materials.add(SNAKE_COLOR)),
                        new_head_transform.with_scale(Vec3::splat(20.)),
                    ))
                    .id(),
            ),
        });

        let prize_transform = transforms.get_mut(game.prize.entity.unwrap()).unwrap();

        if prize_transform.translation != new_head_transform.translation {
            commands
                .entity(game.parts.back().unwrap().entity.unwrap())
                .despawn_recursive();

            game.parts.pop_back().unwrap();
        } else {
            if let Some(_) = game.prize.entity {
                commands
                    .entity(game.prize.entity.unwrap())
                    .despawn_recursive();

                game.prize.entity = None;
            }
        }
    }
}

fn spawn_prize(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    timer: Res<MoveTimer>,
) {
    if let Some(_) = game.prize.entity {
        return;
    }
    if timer.0.finished() {
        game.prize.entity = Some(
            commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(PRIZE_COLOR)),
                    Transform::from_xyz(-60.0, 100.0, 0.0).with_scale(Vec3::splat(20.)),
                ))
                .id(),
        );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_plugins(HelloPlugin)
        .run();
}
