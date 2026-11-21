use bevy::prelude::*;

use crate::ui::{
    debug::UiDebugMode,
    options::{buttons::UiChangeSimButton, popup::UiChangeSimPopup},
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
