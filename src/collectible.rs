use crate::char::Player;
use bevy::prelude::*;

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_collectibles_in_magnet_range, collect_collectible),
        );
    }
}

#[derive(Component)]
pub struct Collectible;

const COLLECT_DEFAULT_RANGE: f32 = 10.0;
const MAGNET_RANGE: f32 = 100.0;

pub fn collect_collectible(
    mut commands: Commands,
    player_position: Query<(&Transform), With<Player>>,
    diamonds: Query<(Entity, &Transform), With<Collectible>>,
) {
    for (diamond_entity, diamond_position) in &mut diamonds.iter() {
        let pposition = player_position.single();

        if diamond_position.translation.distance(pposition.translation) < COLLECT_DEFAULT_RANGE {
            commands.entity(diamond_entity).despawn();
        }
    }
}

fn move_collectibles_in_magnet_range(
    mut collectibles: Query<(&mut Transform), (With<Collectible>, Without<Player>)>,
    player: Query<(&Transform), With<Player>>,
    time: Res<Time>,
) {
    let player_position = player.single().translation;

    for (mut collectible_position) in &mut collectibles.iter_mut() {
        if player_position.distance(collectible_position.translation) <= MAGNET_RANGE {
            let direction = player_position - collectible_position.translation;
            collectible_position.translation +=
                direction.normalize() * 100.0 * time.delta_seconds();
        }
    }
}
