use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin},
};

mod camera_and_background;
mod resources;
mod simulations;
mod ui;

#[derive(Resource)]
pub struct EnvFlags {
    fps_logs: bool,
}

fn main() {
    let mut app = App::new();

    // Move to Mailbox for vsync
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            visible: true,
            present_mode: PresentMode::Mailbox,
            ..default()
        }),
        ..default()
    }));

    // Add environmental flags
    let log_fps = std::env::args().any(|arg| arg == "--fps");
    app.insert_resource(EnvFlags { fps_logs: log_fps });

    // Default, UI, and timings
    app.add_plugins((
        ui::ui::UiPlugin,
        resources::timing::TimingPlugin,
        simulations::selector::SelectorPlugin,
    ));

    // Camera
    app.add_plugins((
        camera_and_background::camera::CameraPlugin,
        camera_and_background::grid::GridPlugin,
    ));

    // Run and unhide window once all plugins are loaded
    app.run();
}
