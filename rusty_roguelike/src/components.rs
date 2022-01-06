pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair, // stores fg and bg color in a single struct
    pub glyph: FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; // Player is an empty struct. This is how some components look like, just a "tag" / description.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy; // Enemy component tag

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String); // struct can also be tuple if it only has 1 element

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity
}