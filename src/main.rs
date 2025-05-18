use macroquad::prelude::*;
mod background;
mod helpers;
mod moving_ball;

#[macroquad::main("Animations")]
async fn main() {
    let mut ball = moving_ball::Ball::new(200.0, 0.0, 15.0, RED, 15.0);
    let mut smaller_ball = moving_ball::Ball::new(25.0, 0.0, 2.0, BLUE, 7.5);

    loop {
        // Check for quit
        if get_keys_pressed().contains(&KeyCode::Escape) {
            return;
        }

        // Draw
        clear_background(BLACK);
        background::draw_background().await;
        smaller_ball = moving_ball::move_and_draw_ball(smaller_ball, Some(ball.pos.clone())).await;
        ball = moving_ball::move_and_draw_ball(ball, None).await;

        // Debug info
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 18.0, WHITE);
        draw_text(&format!("Time: {:.2}", get_time()), 10.0, 40.0, 18.0, WHITE);

        // Next frame
        next_frame().await
    }
}
