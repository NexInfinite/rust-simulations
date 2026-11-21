#![recursion_limit = "256"]
#![warn(clippy::type_complexity)]

use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin},
};

use crate::resources::global::main_in_focus;

mod camera_and_background;
mod resources;
mod simulations;
mod ui;

#[derive(Resource)]
pub struct EnvFlags {
    fps_logs: bool,
    ticks_logs: bool,
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
    app.insert_resource(add_environmental_flags());

    // Default, UI, and timings
    app.add_plugins((ui::plugin::UiPlugin, simulations::plugin::SimulationsPlugin));

    // Resources and systems
    app.add_plugins((
        resources::simulation_timing::SimulationTimingPlugin,
        resources::tick::AppTickPlugin,
        resources::global::GlobalResourcePlugin,
    ));
    app.add_systems(Update, (window_exit).run_if(main_in_focus));

    // Camera
    app.add_plugins((
        camera_and_background::camera::CameraPlugin,
        camera_and_background::grid::GridPlugin,
    ));

    // Finally run the app
    app.run();
}

fn add_environmental_flags() -> EnvFlags {
    EnvFlags {
        fps_logs: std::env::args().any(|arg| arg == "--fps"),
        ticks_logs: std::env::args().any(|arg| arg == "--ticks"),
    }
}

fn window_exit(input: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
