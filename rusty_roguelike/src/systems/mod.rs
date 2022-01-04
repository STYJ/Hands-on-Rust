mod player_input;
mod map_render;
mod entity_render;
mod collisions;

use crate::prelude::*;

// Systems can be run concurrently and are managed by a scheduler
// Generally systems cant see inside other systems thereby making them ideal for module based organisation
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system()) // handle player input
        .add_system(map_render::map_render_system()) // render map
        .add_system(entity_render::entity_render_system()) // render entities that have the Point and Render components
        .add_system(collisions::collisions_system())
        .build()
}