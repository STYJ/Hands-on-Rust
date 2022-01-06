use crate::prelude::*;

// this line annotates this function w/ the procedural macro called system
// this automatically transfers player_input into player_input_system and wraps
// it with whatever extra code is needed for legion to construct a system.
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn player_input(
    ecs: &SubWorld,
    commands: &mut CommandBuffer, // use command buffer to update component instead of writing to it directly.
    #[resource] key : &Option<VirtualKeyCode>,
    #[resource] turn_state : &mut TurnState
) {
    // <>::() is called the turbo fish syntax
    // <(components, to, include)>::query() returns all entities
    // that have these components
    // This query returns mutable references
    // query filters can only require that the component exists but not refer to its contents.
    // You have to use the iterator's filter function instead
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)) )
                .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x !=0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| {
                    **pos == destination
                })
                .for_each(|(entity, _) | {
                    hit_something = true;

                    commands
                        .push(((), WantsToAttack{
                            attacker: player_entity,
                            victim: *entity,
                        }));
                });

            if !hit_something {
                commands
                    .push(((), WantsToMove{
                        entity: player_entity,
                        destination
                    })); // legion's push function don't work for single components so you need an empty tuple
            }
        }
        *turn_state = TurnState::PlayerTurn;  // This is the global state in ECS.resources
    }
}