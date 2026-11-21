use super::{circular, linear};
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum SimulationSceneEnum {
    None,
    Circular,
    Linear,
}

impl SimulationSceneEnum {
    pub const ALL: [SimulationSceneEnum; 3] = [
        SimulationSceneEnum::None,
        SimulationSceneEnum::Circular,
        SimulationSceneEnum::Linear,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            SimulationSceneEnum::None => "None",
            SimulationSceneEnum::Circular => "Circular",
            SimulationSceneEnum::Linear => "Linear",
        }
    }
}

pub struct SimulationsPlugin;

impl Plugin for SimulationsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(SimulationSceneEnum::None);
        app.add_plugins(circular::CircularSimulation);
        app.add_plugins(linear::LinearSimulation);
    }
}
