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

pub fn main_in_focus(global: Res<GlobalResource>) -> bool {
    global.focus == FocusEnum::Main
}

pub fn change_sim_popup_in_focus(global: Res<GlobalResource>) -> bool {
    global.focus == FocusEnum::ChangeSim
}

// Constants for styling

pub const FONT_LARGE: FontSize = FontSize::Px(20.0);
pub const FONT_SMALL: FontSize = FontSize::Px(16.0);

pub const PADDING_LARGE: UiRect = UiRect::axes(Val::Px(15.0), Val::Px(7.5));
#[allow(dead_code)]
pub const PADDING_SMALL: UiRect = UiRect::axes(Val::Px(10.0), Val::Px(5.0));
