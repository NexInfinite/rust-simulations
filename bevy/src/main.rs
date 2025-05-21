use bevy::prelude::*;
mod camera;
mod circular;
mod grid;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            camera::CameraPlugin,
            grid::GridPlugin,
            circular::CircularPlugin,
        ))
        .run();
}
