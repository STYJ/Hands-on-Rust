use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera
) {
    let mut positions = <(Entity, &Point, &Name)>::query(); // find all entities that owns a Point and a Name component
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset; // mouse's global position
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos ) // filter is like if but it's more readable. Basically filter the entities whose position is the same as my mouse position.
        .for_each(|(entity, _, name) | {
            let screen_pos = *mouse_pos * 4; // tooltip layer is 4x larger
            let display = if let Ok(health) = ecs.entry_ref(*entity) // use entry_ref to access entity's component outside of query. If let syntax lets you read the content of this component (Health) if it exists if not ignore it.
                .unwrap()
                .get_component::<Health>() 
            {
                format!("{} : {} hp", &name.0, health.current)// (9)
            } else {
                name.0.clone() // if player is examining something that doesn't have a health component, just return the name. Clone is used to make a copy of the string instead of just borrowing it.
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
