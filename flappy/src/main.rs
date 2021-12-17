use bracket_lib::prelude::*; // use everything that is exported in the prelude

struct State {
    player: Player,
    frame_time: f32, // you need this to track time between frames to control game speed
    mode: GameMode,
}

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0; // try modifying it and see what happens!

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); // similar to cls
        self.frame_time += ctx.frame_time_ms; // the tick function runs as fast as it can so you need to slow the game down. ctx.frame_time_ms = time since last tick() was called.
        if self.frame_time > FRAME_DURATION { // if it exceeds frame duration, run physics simulation and reset frame_time to 0
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key { // this is not restricted by frame_time.
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0,0, "Press SPACE to flap.");
        if self.player.y > SCREEN_HEIGHT { // this is when player has fallen off the screen. remember top left is 0,0
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
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

impl GameState for State {
    // syntax for implementing traits, similar to that of implementing methods for types i.e. "impl traits for type" vs "impl type"
    fn tick(&mut self, ctx: &mut BTerm) {
        // &mut self allows the function tick to modify itself (State), ctx is a window into the currently running bracket-terminal (BTerm)
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // Sets the character on screen
        // to_cp437 converts a unicode symbol to the matching codepage 437 char number
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2; // apply downward momentum if velocity is < 2
        }

        self.y += self.velocity as i32; // cast float to i32 (round down) and add to y coordinate
        self.x += 1; // keep incrementing x as you are moving right

        if self.y < 0 { // ensure that character doesn't fly off screen lol.
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0; // everytime you flap, you set velocity to -2.0 (cancels momentum)
    }
}

fn main() -> BError {
    // This is using the "builder" pattern i.e. successful calls to the builder object
    // adds info to the build request
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?; // request a simple 80x50 terminal, update title and build to finalize the instantiation
    main_loop(context, State::new()) // start executing the game loop and link the game state so bracket-lib knows where tick fn is located at.
}
