use bevy::prelude::*;

use crate::ui::{
    controls::bottom_bar::UiBottomBar, debug::UiDebugMode, options::popup::UiOptionsPopup,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiBottomBar);
        app.add_plugins(UiOptionsPopup);
        app.add_plugins(UiDebugMode);
    }
}
