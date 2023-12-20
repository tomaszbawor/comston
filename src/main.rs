use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins((DefaultPlugins))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("char.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(100.0)),
            ..default()
        },
        texture,
        ..default()
    });
}
