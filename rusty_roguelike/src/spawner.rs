use crate::prelude::*;

// Takes a mutable ref. to world cause world needs to be updated
pub fn spawn_player(ecs: &mut World, pos: Point) {
    // push creates a new Entity composed of the listed components
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            },
            Health {
                current: 20, max: 20
            }
        )
    );
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc()
    };
    ecs.push(
        (
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph
            },
            MovingRandomly{}, // Super cool how you can just add a new component, the logic in systems and register the system and it all works.
            Health{current: hp, max: hp},
            Name(name)
        )
    );
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}