use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::helpers::{self, normalize_x_y};

pub struct Ball {
    pos: helpers::Pos,
    radius: f32,
    time_period: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, time_period: f32) -> Ball {
        Ball {
            pos: helpers::Pos { x: x, y: y },
            radius: (x * x + y * y).sqrt(),
            time_period: time_period,
        }
    }
}

pub async fn move_and_draw_ball(mut ball: Ball) -> Ball {
    ball = move_center_ball(ball).await;
    draw_center_ball(&ball).await;
    return ball;
}

async fn draw_center_ball(ball: &Ball) {
    let pos = normalize_x_y(ball.pos);
    draw_circle(pos.x, pos.y, 10.0, RED);
}

async fn move_center_ball(mut ball: Ball) -> Ball {
    let angle = (2.0 * PI) / ball.time_period * get_time() as f32;
    ball.pos.x = ball.radius * f32::cos(angle);
    ball.pos.y = ball.radius * f32::sin(angle);
    return ball;
}
