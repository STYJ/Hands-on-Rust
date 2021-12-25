mod map; // import the map module, use is for redeclaring path (for convenience)

// declare a "local" module called prelude, since this is neighbours w/ main, you don't have to import it again
// In this prelude module, you are re-exporting some stuff like bracket_lib::prelude::*
// Quite cool that you can do crate::map::* here.
// Note: modules branching frmo crate are visible throughout the entire program.
// You declare this here so you can use it in other files too.
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

use prelude::*;

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GameState for State { // Remember you need to implement the GameState trait so bracket-lib knows where to find the tick function
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Rusty Roguelike").with_fps_cap(30.0).build()?;

    main_loop(context, State::new()) // run the game!
}
