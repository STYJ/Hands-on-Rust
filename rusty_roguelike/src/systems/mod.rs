mod player_input;
mod map_render;
mod entity_render;
mod collisions;
mod random_move;

use crate::prelude::*;

// Systems can be run concurrently and are managed by a scheduler
// Generally systems cant see inside other systems thereby making them ideal for module based organisation
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system()) // handle player input
        .add_system(collisions::collisions_system())
        .flush() // when collisions are detected, it doesn't take into effect immediately. There's a hidden flush at the end of the systems. Flushing after collision detect ensures that any deleted entities are gone before they are rendered. Flushing also ensures that all systems up to that point has been executed before the next one starts. It's good for taming multi threading issues.
        .add_system(map_render::map_render_system()) // render map
        .add_system(entity_render::entity_render_system()) // render entities that have the Point and Render components
        .add_system(random_move::random_move_system())
        .build()
}