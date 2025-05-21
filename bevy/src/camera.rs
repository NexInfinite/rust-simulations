use bevy::{prelude::*, reflect::TypePath};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
        app.add_systems(Update, camera_controls);
    }
}

// Setup a simple 2d scene
pub fn camera_setup(mut commands: Commands) {
    // Camera and window
    commands.spawn((Camera2d, CustomCamera {}));
}

pub fn camera_controls(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    // Exiting
    if input.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

#[derive(TypePath, Component)]
pub struct CustomCamera {}
