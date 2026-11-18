use bevy::{prelude::*, reflect::TypePath};

use crate::resources::global::main_in_focus;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
        app.add_systems(Update, (camera_controls).run_if(main_in_focus));
    }
}

// Setup a simple 2d scene
fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2d, CustomCamera));
}

fn camera_controls(input: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if input.pressed(KeyCode::Escape) && input.pressed(KeyCode::ShiftLeft) {
        exit.write(AppExit::Success);
    }
}

#[derive(TypePath, Component)]
struct CustomCamera;
