use crate::prelude::*;
// usize is either u32 or u64 depending on your machine.
// usize is used to index collections and arrays
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

// Clone allows you to do myTile.clone() which does a deep clone.
// Copy allows you to copy a type instead of moving the ownership around
// PartialEq lets you do ==
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    // this enum is available to anyone that does "use crate::prelude::*" since prelude is created by importing all of map
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES], // initialise an array with NUM_TILES number of floor tiles.
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        // checks if the player is within the boundary of the map
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        // checks if adventurer is within the boundary of the map and tile is a floor
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

// Transforming (x,y) into a single index is known as striding
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}