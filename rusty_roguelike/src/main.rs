mod map; // import the map module, use is for redeclaring path (for convenience)
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;

// declare a "local" module called prelude, since this is neighbours w/ main, you don't have to import it again
// In this prelude module, you are re-exporting some stuff like bracket_lib::prelude::*
// Quite cool that you can do crate::map::* here.
// Note: modules branching frmo crate are visible throughout the entire program.
// You declare this here so you can use it in other files too.
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

// ecs = entity-component-system
// entity = stuff in game, no logic, usually just an ID
// component = properties of an entity e.g. position, health, movement AI. Note: components don't have logic as well! They serve more like descriptions.
// system = query the entities and components and provide one element of gameplay. All logic resides in system.
struct State {
    ecs: World,
    resources: Resources, // map and camera are resources
    systems: Schedule
}

impl State {
    fn new() -> Self {
        // Default constructors
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start); // spawns player in world
        // Instead of storing map builder in the state, you insert into resources
        // spawn monsters in the center of each room (excl. the first one), 
        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs,&mut rng, pos));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        Self {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
}

impl GameState for State {
    // Remember you need to implement the GameState trait so bracket-lib knows where to find the tick function
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key); // this makes the keyboard input available for any system that requests it
        self.systems.execute(&mut self.ecs,&mut self.resources);
        render_draw_buffer(ctx).expect("Render error"); // when all the batches are ready to render, render it
    }
}

fn main() -> BError {
    let context = BTermBuilder::new() // create a generic terminal and specify attributes
        .with_title("Dungeon Crawler") 
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // specifies the size of subsequent consoles
        .with_tile_dimensions(32, 32) // size of each character in your font file
        .with_resource_path("resources/") // directory of font
        .with_font("dungeonfont.png", 32, 32) // name of font and dimensions, usually same as tile dimensions
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Add a console using the specified dimensions and the named tile graphics file
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Add a console with no background so transparency shows through
        .build()?;
    // Note: you are not rendering the entire map at once, so you need to use a camera

    main_loop(context, State::new()) // run the game!
}
