use bracket_lib::prelude::*; // use everything that is exported in the prelude

struct State {
    mode: GameMode
}

enum GameMode {
    Menu,
    Playing,
    End
}

impl State {
    fn new () -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: fill this later
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        // TODO: fill this later
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        // Display menu, respond to user input
        // Change mode to playing
        // Reset all game states
        ctx.cls(); // ctx = context, cls = clear
        ctx.print_centered(5, "Welcome to Flappy Dragon"); // print_centered(y_coordinate, text)
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!!");
        ctx.print_centered(8, "(P) Play again");
        ctx.print_centered(9, "(Q) Quit Game!");
        
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State { // syntax for implementing traits, similar to that of implementing methods for types i.e. "impl traits for type" vs "impl type"
    fn tick(&mut self, ctx: &mut BTerm) { // &mut self allows the function tick to modify itself (State), ctx is a window into the currently running bracket-terminal (BTerm)
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx)
        }
    }
}

fn main() -> BError {
    // This is using the "builder" pattern i.e. successful calls to the builder object
    // adds info to the build request
    let context = BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?; // request a simple 80x50 terminal, update title and build to finalize the instantiation
    main_loop(context, State::new()) // start executing the game loop and link the game state so bracket-lib knows where tick fn is located at.
}
