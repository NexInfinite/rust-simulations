use macroquad::prelude::*;

const STEP: f32 = 100.0;
#[derive(Clone, Copy)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

pub fn get_midway_width() -> f32 {
    (screen_width() / STEP / 2.0).round() * STEP
}

pub fn get_midway_height() -> f32 {
    (screen_height() / STEP / 2.0).round() * STEP
}

pub fn get_step() -> f32 {
    STEP
}

pub fn normalize_x_y(pos: Pos) -> Pos {
    let mut local_pos = pos.clone();
    local_pos.x = get_midway_width() + local_pos.x;
    local_pos.y = get_midway_height() + local_pos.y;
    return local_pos;
}
