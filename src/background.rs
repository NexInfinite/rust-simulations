use macroquad::prelude::*;

use crate::helpers::{get_midway_height, get_midway_width, get_step};

pub async fn draw_background() {
    let line_thickness = 0.5;
    let step = get_step();

    for i in (0..screen_width() as i32).step_by(step as usize) {
        draw_line(
            i as f32,
            0.0,
            i as f32,
            screen_height(),
            line_thickness,
            find_color(i, get_midway_width()),
        );
    }

    for i in (0..screen_height() as i32).step_by(step as usize) {
        draw_line(
            0.0,
            i as f32,
            screen_width(),
            i as f32,
            line_thickness,
            find_color(i, get_midway_height()),
        );
    }

    fn find_color(i: i32, midway: f32) -> Color {
        if i as f32 == midway {
            return WHITE;
        } else {
            return DARKGRAY;
        }
    }
}
