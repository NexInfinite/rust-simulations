use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use iyes_perf_ui::prelude::*;
use std::sync::LazyLock;

use crate::EnvFlags;

pub struct UiDebugMode;

static START: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);

impl Plugin for UiDebugMode {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
            PerfUiPlugin,
        ));
        app.add_systems(
            Update,
            keyboard_handler.before(iyes_perf_ui::PerfUiSet::Setup),
        );
        app.add_systems(Update, log_fps);
    }
}

fn log_fps(frames: Res<DiagnosticsStore>, env_flags: Res<EnvFlags>) {
    // Only log if fps flag is set
    if !env_flags.fps_logs {
        return;
    }

    if let Some(value) = frames
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.value())
    {
        let elapsed = START.elapsed();

        println!(
            "{:02}:{:03} - {:.1} fps",
            elapsed.as_secs(),
            elapsed.subsec_millis(),
            value
        );
    }
}

fn keyboard_handler(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    debug_plugin_query: Query<Entity, With<PerfUiRoot>>,
) {
    if input.just_pressed(KeyCode::F1) {
        if let Ok(e) = debug_plugin_query.single() {
            commands.entity(e).despawn();
        } else {
            commands.spawn(PerfUiAllEntries::default());
        }
    }
}
