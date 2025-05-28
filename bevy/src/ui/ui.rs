use bevy::prelude::*;

use super::{change_sim::UiChangeSim, play_pause_button::UiPlayPauseButton};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlayPauseButton);
        app.add_plugins(UiChangeSim);
    }
}
