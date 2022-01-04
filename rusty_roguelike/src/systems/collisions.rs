use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // command buffers are like post system scripts e.g. removing enemies after the system is done processing it.
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos); // this looks weird but it's correct. There's only 1 player so only 1 player_pos. The purpose of this step is to get player's position
    let mut enemies = <(Entity, &Point)>::query() // get all enemies
        .filter(component::<Enemy>());
    enemies // check if any enemies are currently colliding w/ player, if so, remove it.
        .iter(ecs)
        .filter(|(_,pos)| **pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        }
    );
}
