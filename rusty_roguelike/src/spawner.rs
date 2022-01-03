use crate::prelude::*;

// Takes a mutable ref. to world cause world needs to be updated
pub fn spawn_player(ecs: &mut World, pos: Point) {
    // push creates a new Entity composed of the listed components
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            },
        )
    );
}
