use bevy::prelude::*;

mod camera_and_background;
mod resources;
mod simulations;
mod ui;

fn main() {
    App::new()
        // Default, UI, and timings
        .add_plugins((
            DefaultPlugins,
            ui::ui::UiPlugin,
            resources::timing::TimingPlugin,
            simulations::selector::SelectorPlugin,
        ))
        // Camera
        .add_plugins((
            camera_and_background::camera::CameraPlugin,
            camera_and_background::grid::GridPlugin,
        ))
        .run();
}
