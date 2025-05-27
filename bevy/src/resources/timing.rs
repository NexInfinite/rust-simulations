use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationTiming {
    pub paused: bool,
    pub time: f32,
    // TODO: Add speed controls and reset function
}

pub struct TimingPlugin;

impl Plugin for TimingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationTiming {
            paused: false,
            time: 0.0,
        });
        app.add_systems(Update, (controls, tick));
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
