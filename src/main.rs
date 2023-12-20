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

#[derive(Component)]
struct Player {
    speed: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("char.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player { speed: 100.0 },
    ));
}

fn char_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transformation, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transformation.translation.y += movement_amount
        }
        if input.pressed(KeyCode::S) {
            transformation.translation.y -= movement_amount
        }
        if input.pressed(KeyCode::A) {
            transformation.translation.x -= movement_amount
        }
        if input.pressed(KeyCode::D) {
            transformation.translation.x += movement_amount
        }
    }
}
