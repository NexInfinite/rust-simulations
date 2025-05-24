use bevy::prelude::*;

mod camera;
mod circular;
mod grid;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ui::UiPlugin,
            camera::CameraPlugin,
            grid::GridPlugin,
            circular::CircularPlugin,
        ))
        .run();
}
