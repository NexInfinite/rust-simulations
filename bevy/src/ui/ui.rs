use bevy::prelude::*;

use crate::ui::{
    change_sim::{button::UiChangeSimButton, popup::UiChangeSimPopup},
    debug::UiDebugMode,
    play_pause_button::UiPlayPauseButton,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlayPauseButton);
        app.add_plugins((UiChangeSimButton, UiChangeSimPopup));
        app.add_plugins(UiDebugMode);
    }
}
