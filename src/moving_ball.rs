use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::helpers::{self, normalize_x_y};

pub struct Ball {
    pub pos: helpers::Pos,
    pub radius: f32,
    pub time_period: f32,
    pub color: Color,
    pub size: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, time_period: f32, color: Color, size: f32) -> Ball {
        Ball {
            pos: helpers::Pos { x: x, y: y },
            radius: (x * x + y * y).sqrt(),
            time_period: time_period,
            color: color,
            size: size,
        }
    }
}

pub async fn move_and_draw_ball(mut ball: Ball, normal_position: Option<helpers::Pos>) -> Ball {
    ball = move_center_ball(ball).await;
    draw_center_ball(&ball, normal_position).await;
    return ball;
}

async fn draw_center_ball(ball: &Ball, normal_position: Option<helpers::Pos>) {
    let mut pos = normalize_x_y(ball.pos);
    if let Some(normal_pos) = normal_position {
        pos.x += normal_pos.x + ball.pos.x;
        pos.y += normal_pos.y + ball.pos.y;
    }
    draw_circle(pos.x, pos.y, ball.size, ball.color);
}

async fn move_center_ball(mut ball: Ball) -> Ball {
    let angle = (2.0 * PI) / ball.time_period * get_time() as f32;
    ball.pos.x = ball.radius * f32::cos(angle);
    ball.pos.y = ball.radius * f32::sin(angle);
    return ball;
}
