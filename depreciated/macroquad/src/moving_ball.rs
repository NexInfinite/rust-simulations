use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::helpers::{self, normalize_x_y};

const TRAIL_LENGTH: usize = 400;

pub struct Ball {
    pub pos: helpers::Pos,
    pub radius: f32,
    pub time_period: f32,
    pub color: Color,
    pub size: f32,
    last_drawn_pos: [helpers::Pos; TRAIL_LENGTH],
}

impl Ball {
    pub fn new(x: f32, y: f32, time_period: f32, color: Color, size: f32) -> Ball {
        Ball {
            pos: helpers::Pos { x: x, y: y },
            last_drawn_pos: [helpers::Pos { x: 0.0, y: 0.0 }; TRAIL_LENGTH],
            radius: (x * x + y * y).sqrt(),
            time_period: time_period,
            color: color,
            size: size,
        }
    }
}

pub async fn move_and_draw_ball(mut ball: Ball, normal_position: Option<helpers::Pos>) -> Ball {
    ball = move_center_ball(ball).await;
    ball.last_drawn_pos[TRAIL_LENGTH - 1] = draw_center_ball(&ball, normal_position).await;
    ball.last_drawn_pos.rotate_right(1);
    draw_trail(ball.last_drawn_pos, ball.color).await;
    return ball;
}

async fn draw_center_ball(ball: &Ball, normal_position: Option<helpers::Pos>) -> helpers::Pos {
    let mut screen_pos = normalize_x_y(ball.pos);
    if let Some(normal_pos) = normal_position {
        screen_pos.x += normal_pos.x + ball.pos.x;
        screen_pos.y += normal_pos.y + ball.pos.y;
    }
    draw_circle(screen_pos.x, screen_pos.y, ball.size, ball.color);
    return screen_pos;
}

async fn draw_trail(history: [helpers::Pos; TRAIL_LENGTH], color: Color) {
    for i in 0..history.len() - 1 {
        let item1 = history[i];
        let item2 = history[i + 1];
        if item1.x != 0.0 && item1.y != 0.0 && item2.x != 0.0 && item2.y != 0.0 {
            draw_line(item1.x, item1.y, item2.x, item2.y, 1.0, color);
        }
    }
}

async fn move_center_ball(mut ball: Ball) -> Ball {
    let angle = (2.0 * PI) / ball.time_period * get_time() as f32;
    ball.pos.x = ball.radius * f32::cos(angle);
    ball.pos.y = ball.radius * f32::sin(angle);
    return ball;
}
