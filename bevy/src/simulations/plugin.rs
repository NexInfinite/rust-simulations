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
    }
}
