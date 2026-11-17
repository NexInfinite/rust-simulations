use bevy::prelude::*;

use crate::EnvFlags;

#[derive(Resource)]
pub struct AppTicks {
    pub tick: i32,      // Ticks up to TICK_RATE, and then back to 0
    previous_tick: f32, // Time of previous tick
}

const TICK_RATE: i32 = 20; // Max number of ticks per second
const EXPECTED_TIME_PER_TICK: f32 = 1.0 / TICK_RATE as f32;

pub struct AppTickPlugin;

impl Plugin for AppTickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AppTicks {
            tick: 0,
            previous_tick: -1.0,
        });
        app.add_systems(Update, tick);
    }
}

fn tick(mut app_tick: ResMut<AppTicks>, time: Res<Time>, env_flags: Res<EnvFlags>) {
    let time_from_start = time.elapsed_secs();

    if app_tick.previous_tick == -1.0 {
        app_tick.previous_tick = time_from_start;
        return;
    }

    // See how many ticks should have been in that elapsed time, a long wait between frames may cover multiple ticks
    let dt = time_from_start - app_tick.previous_tick;
    if dt >= EXPECTED_TIME_PER_TICK {
        let elapsed_ticks = (dt / EXPECTED_TIME_PER_TICK) as i32;
        app_tick.tick = (app_tick.tick + elapsed_ticks) % TICK_RATE;
        app_tick.previous_tick = time_from_start;

        if env_flags.ticks_logs {
            println!("Tick: {}, \tElapsed: {:.2}", app_tick.tick, dt);
        }
    }
}
