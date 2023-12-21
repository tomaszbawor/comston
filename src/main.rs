mod char;
mod diamond;

use crate::char::{char_movement, PlayerPlugin};
use crate::diamond::DiamondPlugin;
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
        .add_plugins((PlayerPlugin, DiamondPlugin))
        .add_systems(Startup, camera_setup)
        .add_systems(Update, char_movement)
        .run()
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
