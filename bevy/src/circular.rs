use bevy::{color::palettes::css::RED, prelude::*};

pub struct CircularPlugin;

impl Plugin for CircularPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ball);
    }
}

pub fn ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(10.0));
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
