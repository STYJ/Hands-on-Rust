pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair, // stores fg and bg color in a single struct
    pub glyph: FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; // Player is an empty struct. This is how some components look like, just a "tag" / description.