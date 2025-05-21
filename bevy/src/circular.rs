use std::f32::consts::PI;

use bevy::{color::palettes::css::RED, prelude::*};

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

fn move_ball(mut query: Query<(&mut Transform, &mut Ball), With<Ball>>, time: Res<Time>) {
    for (mut transform, ball) in &mut query {
        let mut translation = transform.translation;
        let angle = (2.0 * PI) / ball.time_period * time.elapsed_secs();
        translation.x = ball.radius * f32::cos(angle);
        translation.y = ball.radius * f32::sin(angle);
        transform.translation = translation;
    }
}
