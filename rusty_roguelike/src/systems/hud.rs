use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>()); // find entities with the health component and filter for player.
    let player_health = health_query
        .iter(ecs)
        .nth(0)
        .unwrap(); // take the 0th element since there's only 1 player and unwrap it to get the underlying element

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, 
        "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal( // a helper method in bracket-lib to help w/ the common task of drawing health bars
        Point::zero(), // where the bar should start
        SCREEN_WIDTH*2, // how long is the bar
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK) // full vs empty health bar
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {} ", 
            player_health.current, 
            player_health.max
        ),
        ColorPair::new(WHITE, RED)
    );
    draw_batch.submit(10000).expect("Batch error");
}
