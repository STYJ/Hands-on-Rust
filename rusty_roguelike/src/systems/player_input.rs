use crate::prelude::*;

// this line annotates this function w/ the procedural macro called system
// this automatically transfers player_input into player_input_system and wraps
// it with whatever extra code is needed for legion to construct a system.
#[system]
#[write_component(Point)] // requests writable access to the Point component
#[read_component(Player)] // requests read access to the Player component
pub fn player_input(
    ecs: &mut SubWorld, // SubWorld is like a world but you can only see the components you requested
    #[resource] map: &Map, // #[resource] is a proc macro to request access for the stuff you inserted into resources
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0)
        };
        if delta.x != 0 || delta.y != 0 {
            // <>::() is called the turbo fish syntax
            // <(components, to, include)>::query() returns all entities
            // that have these components
            // This query returns mutable references
            let mut players = <&mut Point>::query()
                .filter(component::<Player>());

            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                } 
            });
        }
    }
}