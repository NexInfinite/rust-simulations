use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FocusEnum {
    Main,
    ChangeSim,
}

#[derive(Resource)]
pub struct GlobalResource {
    pub focus: FocusEnum,
}

pub struct GlobalResourcePlugin;

impl Plugin for GlobalResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalResource {
            focus: FocusEnum::Main,
        });
    }
}
