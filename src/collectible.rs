use crate::char::Player;
use crate::diamond::Position;
use bevy::prelude::*;

#[derive(Component)]
pub struct Collectible;

pub fn collect_collectible(
    mut commands: Commands,
    mut player: Query<(&Player, &Transform)>,
    diamonds: Query<(Entity, &Collectible, &Position)>,
) {
    for diamond in &mut diamonds.iter() {
        for player in &mut player.iter() {
            if diamond
                .2
                 .0
                .distance(Vec2::from([player.1.translation.x, player.1.translation.y]))
                < 25.0
            {
                commands.entity(diamond.0).despawn();
            }
        }
    }
}
