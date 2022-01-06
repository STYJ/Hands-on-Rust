mod player_input;
mod map_render;
mod entity_render;
// mod collisions;
mod random_move;
mod end_turn;
mod movement;
mod hud;
mod tooltips;
mod combat;

use crate::prelude::*;

// Systems can be run concurrently and are managed by a scheduler
// Generally systems cant see inside other systems thereby making them ideal for module based organisation
// While awaiting player input, it calls player_input_system and screen still needs to display map and entities 
pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system()) // handle player input
        .flush()
        .add_system(map_render::map_render_system()) // render map
        .add_system(entity_render::entity_render_system()) // render entities that have the Point and Render components
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

// When it's the player's turn, game doesn't accept input but checks for collisions and renders map and entities and finishes w/ endturn system
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush() // try with and without flushing! When collisions are detected, it doesn't take into effect immediately. There's a hidden flush at the end of the systems. Flushing after collision detect ensures that any deleted entities are gone before they are rendered. Flushing also ensures that all systems up to that point has been executed before the next one starts. It's good for taming multi threading issues.
        .add_system(map_render::map_render_system()) // render map
        .add_system(entity_render::entity_render_system()) // render entities that have the Point and Render components
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build() 
}

// When it's the monster's turn, game doesn't accept input but instead does a random move, checks for collisions and renders maps / entities and finishes w/ endturn system
pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
