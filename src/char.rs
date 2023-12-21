use bevy::app::{App, Plugin};
use bevy::input::Input;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, char_movement);
    }
}

const PLAYER_INIT_SPEED: f32 = 100.0;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("char.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(50.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player {
            speed: PLAYER_INIT_SPEED,
        },
    ));
}

pub fn char_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transformation, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        let mut movement_vector = Vec2::new(0.0, 0.0);
        // calculate movement vector

        input.get_pressed().for_each(|key| match key {
            KeyCode::W => movement_vector.y += movement_amount,
            KeyCode::S => movement_vector.y -= movement_amount,
            KeyCode::A => movement_vector.x -= movement_amount,
            KeyCode::D => movement_vector.x += movement_amount,
            _ => (),
        });

        // normalize movement vector
        if movement_vector.x != 0.0 && movement_vector.y != 0.0 {
            movement_vector.x = movement_vector.x / 2.0_f32.sqrt();
            movement_vector.y = movement_vector.y / 2.0_f32.sqrt();
        }

        // apply movement vector
        transformation.translation += movement_vector.extend(0.0);
    }
}
