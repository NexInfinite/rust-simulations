use bevy::prelude::*;

use super::play_pause_button;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(play_pause_button::UiPlayPauseButton);
    }
}
