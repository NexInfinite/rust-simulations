use bevy::prelude::*;

use super::{circular, linear};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimulationState {
    Circular,
    Linear,
}

pub struct SelectorPlugin;

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(SimulationState::Linear);
        app.add_plugins(circular::CircularSimulation);
        app.add_plugins(linear::LinearSimulation);

        app.add_systems(Update, controls);
    }
}

fn controls(
    input: Res<ButtonInput<KeyCode>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if input.just_pressed(KeyCode::Digit1) {
        next_sim_state.set(SimulationState::Circular);
    } else if input.just_pressed(KeyCode::Digit2) {
        next_sim_state.set(SimulationState::Linear);
    }
}
