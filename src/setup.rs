use crate::config::{PRIZE_COLOR, SNAKE_HEAD_COLOR};
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct SetupPlugin;

#[derive(Resource)]
pub struct MoveTimer(pub Timer);

#[derive(Default)]
pub struct Prize {
    pub entity: Option<Entity>,
}

#[derive(Default, Debug)]
pub struct Part {
    pub entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct Game {
    pub prize: Prize,
    pub parts: VecDeque<Part>,
    pub direction: Direction,
    pub direction_changed: bool,
}

#[derive(Default, PartialEq, Eq)]
pub enum Direction {
    Left,
    #[default]
    Right,
    Up,
    Down,
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)));
        app.init_resource::<Game>();
        app.add_systems(Startup, setup);
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
                    MeshMaterial2d(materials.add(SNAKE_HEAD_COLOR)),
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
