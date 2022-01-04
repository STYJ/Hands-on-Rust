use crate::prelude::*;

#[system]
pub fn map_render(
    #[resource] map: &Map, // remember to get stuff from resources, you need to specify the prog macro #[resource]
    #[resource] camera: &Camera
) {
    // Systems automatically turn your game multi threaded allowing systems to be executed concurrently
    // Multiple systems writing to the terminal at the same time is bad 
    // Bracket-lib doesn't have a locking system but instead offers a batching service
    // You can request a new batch by calling DrawBatch::new()
    // This creates a buffer of deferred rendering commands i.e. renderring is stored first and not executed
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0); // target base layer
    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x ..= camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(
                    pt - offset,
                    ColorPair::new(
                        WHITE,
                        BLACK
                    ),
                    glyph
                );
            }
        }
    }
    // the sort_order of 0 ensures that the map is drawn at the beginning of the render cycle.
    draw_batch.submit(0).expect("Batch error");
}