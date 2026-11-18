use super::{circular, linear};
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimulationSceneEnum {
    None,
    Circular,
    Linear,
}

pub struct SimulationsPlugin;

impl Plugin for SimulationsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(SimulationSceneEnum::None);
        app.add_plugins(circular::CircularSimulation);
        app.add_plugins(linear::LinearSimulation);
        app.add_systems(Update, controls);
    }
}

fn controls(
    input: Res<ButtonInput<KeyCode>>,
    mut next_sim_state: ResMut<NextState<SimulationSceneEnum>>,
) {
    if input.just_pressed(KeyCode::Digit1) {
        next_sim_state.set(SimulationSceneEnum::Circular);
    } else if input.just_pressed(KeyCode::Digit2) {
        next_sim_state.set(SimulationSceneEnum::Linear);
    }
}
