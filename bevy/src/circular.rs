use std::f32::consts::PI;

use bevy::{color::palettes::css::RED, prelude::*};

use crate::grid;

pub struct CircularPlugin;
#[derive(Component)]
pub struct Ball {
    radius: f32,
    time_period: f32,
}

impl Plugin for CircularPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ball);
        app.add_systems(Update, move_ball);
    }
}

fn ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(10.0));
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(0.0, 100.0, 1.0),
        Ball {
            radius: 100.0,
            time_period: 5.0,
        },
    ));
}

fn move_ball(
    mut ball_query: Query<(&mut Transform, &mut Ball), With<Ball>>,
    mut grid_shader_materials: ResMut<Assets<grid::GridShader>>,
    time: Res<Time>,
) {
    let mut zoom = 1.0;
    for material in grid_shader_materials.iter_mut() {
        zoom = material.1.default_zoom / material.1.zoom;
    }

    for (mut transform, ball) in &mut ball_query {
        // Move Ball
        let mut translation = transform.translation;
        let angle = (2.0 * PI) / ball.time_period * time.elapsed_secs();
        translation.x = ball.radius * zoom * f32::cos(angle);
        translation.y = ball.radius * zoom * f32::sin(angle);
        transform.translation = translation;

        // Resize Ball
        transform.scale = vec3(zoom, zoom, 1.0);
    }
}
