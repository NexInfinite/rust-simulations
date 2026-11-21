use bevy::{prelude::*, reflect::TypePath};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

// Setup a simple 2d scene
fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2d, CustomCamera));
}

#[derive(TypePath, Component)]
struct CustomCamera;
