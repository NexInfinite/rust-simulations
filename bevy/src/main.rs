use bevy::prelude::*;

mod camera;
mod circular;
mod grid;
mod timing;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            timing::TimingPlugin,
            ui::UiPlugin,
            camera::CameraPlugin,
            grid::GridPlugin,
            circular::CircularPlugin,
        ))
        .run();
}
