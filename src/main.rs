use macroquad::prelude::*;
mod background;
mod helpers;
mod moving_ball;

#[macroquad::main("Animations")]
async fn main() {
    let mut ball = moving_ball::Ball::new(200.0, 0.0, 5.0);

    loop {
        // Check for quit
        if get_keys_pressed().contains(&KeyCode::Escape) {
            return;
        }

        // Draw
        clear_background(BLACK);
        background::draw_background().await;
        ball = moving_ball::move_and_draw_ball(ball).await;

        // Debug info
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 18.0, WHITE);
        draw_text(&format!("Time: {:.2}", get_time()), 10.0, 40.0, 18.0, WHITE);

        next_frame().await
    }
}
