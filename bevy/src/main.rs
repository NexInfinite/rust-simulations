use bevy::{prelude::*, sprite::Material2dPlugin};
mod camera;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, camera::CameraPlugin))
        .run();
}
