use bevy::prelude::*;

use crate::resources::global::main_in_focus;

#[derive(Resource)]
pub struct SimulationTiming {
    pub paused: bool,
    pub time: f32,
    // TODO: Add speed controls and reset function
}

pub struct SimulationTimingPlugin;

impl Plugin for SimulationTimingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationTiming {
            paused: true,
            time: 0.0,
        });
        app.add_systems(Update, (controls, tick).run_if(main_in_focus));
    }
}

fn controls(input: Res<ButtonInput<KeyCode>>, mut simulation_timer: ResMut<SimulationTiming>) {
    if input.just_pressed(KeyCode::Space) {
        simulation_timer.paused = !simulation_timer.paused;
    }
}

fn tick(mut simulation_timer: ResMut<SimulationTiming>, time: Res<Time>) {
    if !simulation_timer.paused {
        simulation_timer.time += time.delta_secs();
    }
}
