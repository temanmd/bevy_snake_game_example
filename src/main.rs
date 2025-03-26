use bevy::prelude::*;
use rand::prelude::*;
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
    direction_changed: bool,
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
        app.insert_resource(MoveTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)));
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (movement_system, update_snake, spawn_prize, check_collide),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game: ResMut<Game>,
) {
    commands.spawn(Camera2d);

    game.direction_changed = false;

    game.parts.push_front(Part {
        entity: Some(
            commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(SNAKE_COLOR)),
                    Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(20.)),
                ))
                .id(),
        ),
    });

    game.prize.entity = Some(
        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(PRIZE_COLOR)),
                Transform::from_xyz(0.0, 200.0, 1.).with_scale(Vec3::splat(20.)),
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

fn check_collide(windows: Query<&mut Window>) {
    // let window = windows.single();
    // println!("{:?}", window.resolution.height());
    // println!("{:?}", window.resolution.width());
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_plugins(HelloPlugin)
        .run();
}
