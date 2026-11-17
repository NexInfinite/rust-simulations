use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

pub struct UiDebugMode;

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
