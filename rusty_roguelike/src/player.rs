use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    // use point instead of x and y because it's already imported from bracket-lib's prelude
    // also can do some basic vector gemetry
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1); // set the player layer
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'), // Draw the player with a white foreground, black background and @ symbol
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        // Checks if a key is of type ctx.key
        // If so, match the key on either one of the movement keys
        // and save the results to delta
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };

            // Calculate new position and check if it is a valid position
            // If ok, move player.
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                // When the player moves, update the camera.
                camera.on_player_move(new_position);
            }
        }
    }
}
