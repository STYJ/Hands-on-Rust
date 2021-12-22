mod map; // import the map module, use is for redeclaring path (for convenience)

// declare a "local" module called prelude, since this is neighbours w/ main, you don't have to import it again
// In this prelude module, you are re-exporting some stuff like bracket_lib::prelude::*
// Quite cool that you can do crate::map::* here.
// Note: modules branching frmo crate are visible throughout the entire program.
// You declare this here so you can use it in other files too.
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH:i32 = 80;
    pub const SCREEN_HEIGHT:i32 = 50;
    pub use crate::map::*;
}

use prelude::*; //

fn main() {
    println!("Hello, world!");
}
