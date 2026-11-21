use bevy::prelude::*;

use crate::ui::{
    debug::UiDebugMode,
    options::{buttons::UiChangeSimButton, popup::UiOptionsPopup},
    play_pause_button::UiPlayPauseButton,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlayPauseButton);
        app.add_plugins((UiChangeSimButton, UiOptionsPopup));
        app.add_plugins(UiDebugMode);
    }
}
