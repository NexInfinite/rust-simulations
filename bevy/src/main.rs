use bevy::prelude::*;

mod camera_and_background;
mod resources;
mod simulations;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            resources::timing::TimingPlugin,
            ui::ui::UiPlugin,
            camera_and_background::camera::CameraPlugin,
            camera_and_background::grid::GridPlugin,
            simulations::selector::SelectorPlugin,
        ))
        .run();
}
