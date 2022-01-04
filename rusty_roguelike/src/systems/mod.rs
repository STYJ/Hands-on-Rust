mod player_input;
use crate::prelude::*;

// Systems can be run concurrently and are managed by a scheduler
// Generally systems cant see inside other systems thereby making them ideal for module based organisation
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .build()
}