use crate::prelude::*;

// The movement system iterates all entities with a WantsToMove component then checks if the move is valid. If it is, update the Point component to the new destination. If the entity is also a player, update camera.

#[system(for_each)]// for systems that only run a single query, this system derives the query from the system parameters (<&WantsToMove>::query().iter() etc.) and runs the system once for every matching entity.
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination); // safer and more efficient to add commands instead of directly modifying component. Adding a component that already exists will replace the old one.

        if ecs.entry_ref(want_move.entity) // accessing components on entities outside of a query is more complicated. use the entry_ref method. Entities are only available if you specify them in read / write component proc macro.
            .unwrap()
            .get_component::<Player>().is_ok() // Effectively this line is saying if the moving entity exists and is a player.
        {
            camera.on_player_move(want_move.destination);// (6)
        }
    }
    commands.remove(*entity); // remove the message once it is processed.
}
