use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Comston".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, char_movement)
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

fn char_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transf, _) in &mut characters {
        if input.pressed(KeyCode::W) {
            transf.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transf.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transf.translation.x -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transf.translation.x += 100.0 * time.delta_seconds();
        }
    }
}
