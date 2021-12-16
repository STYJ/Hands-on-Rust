use bracket_lib::prelude::*; // use everything that is exported in the prelude

struct State {}

impl GameState for State { // syntax for implementing traits, similar to that of implementing methods for types i.e. "impl traits for type" vs "impl type"
    fn tick(&mut self, ctx: &mut BTerm) { // &mut self allows the function tick to modify itself (State), ctx is a window into the currently running bracket-terminal (BTerm)
        ctx.cls(); // ctx = context, cls = clear
        ctx.print(1, 1, "hello, bracket terminal"); // coordinates to print string, (0,0) is top left corner
    }
}

fn main() -> BError {
    // This is using the "builder" pattern i.e. successful calls to the builder object
    // adds info to the build request
    let context = BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?; // request a simple 80x50 terminal, update title and build to finalize the instantiation
    main_loop(context, State{}) // start executing the game loop and link the game state so bracket-lib knows where tick fn is located at.
}
