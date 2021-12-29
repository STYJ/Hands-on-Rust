use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map, // MapBuilder creates its own map and passes the result to game
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile:TileType) {
        // iter_mut returns mutable references so if you want to modify it, you need to dereference it.
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
}