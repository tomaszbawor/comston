use crate::collectible::{collect_collectible, Collectible};
use bevy::prelude::*;

pub struct DiamondPlugin;

impl Plugin for DiamondPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_diamond_spawner)
            .add_systems(Update, (spawn_diamonds, collect_collectible));
    }
}

#[derive(Component)]
struct Diamond {
    timer: Timer,
}

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Resource)]
struct DiamondSpawnerConfig {
    timer: Timer,
}

const MAX_DIAMOND_COUNT: usize = 100;

fn spawn_diamonds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    diamonds: Query<&Diamond>,
    mut config: ResMut<DiamondSpawnerConfig>,
    time: Res<Time>,
) {
    // tick timer
    config.timer.tick(time.delta());

    // check if timer is ready
    if !config.timer.finished() {
        return;
    }

    if diamonds.iter().len() >= MAX_DIAMOND_COUNT {
        return;
    }

    let texture = asset_server.load("diamond.png");

    // randomize position
    let x_pos = rand::random::<f32>() * 640.0 - 320.0;
    let y_pos = rand::random::<f32>() * 480.0 - 240.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(30.0)),
                ..default()
            },
            texture,
            transform: Transform::from_xyz(x_pos, y_pos, 0.0),
            ..default()
        },
        Diamond {
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        },
        Collectible,
        Position(Vec2::new(x_pos, y_pos)),
    ));
}

fn setup_diamond_spawner(mut commands: Commands) {
    commands.insert_resource(DiamondSpawnerConfig {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}
