use bevy::{
    color::palettes::css::{GREEN, RED},
    platform::collections::HashMap,
    prelude::*,
};
use std::f32::consts::PI;

use super::selector::SimulationState;
use crate::{camera_and_background::grid, resources::timing::SimulationTiming};

pub struct CircularSimulation;

impl Plugin for CircularSimulation {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Circular), spawn_balls);
        app.add_systems(OnExit(SimulationState::Circular), despawn_balls);
        app.add_systems(
            Update,
            (
                scale_ball.run_if(in_state(SimulationState::Circular)),
                move_ball.run_if(in_state(SimulationState::Circular)),
            ),
        );
    }
}

#[derive(Component, Clone, Copy)]
pub struct Ball {
    radius: f32,
    time_period: f32,
    follow_id: Option<i32>,
    id: i32,
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn balls
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(100.0, 0.0, 1.0),
        Ball {
            radius: 100.0,
            time_period: 5.0,
            follow_id: None,
            id: 0,
        },
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        Transform::from_xyz(150.0, 0.0, 1.0),
        Ball {
            radius: 50.0,
            time_period: 1.0,
            follow_id: Some(0),
            id: 1,
        },
    ));
}

fn despawn_balls(mut commands: Commands, ball_query: Query<Entity, With<Ball>>) {
    for entity in ball_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn scale_ball(
    mut ball_query: Query<&mut Transform, With<Ball>>,
    mut grid_shader_materials: ResMut<Assets<grid::GridShader>>,
) {
    let mut zoom = 1.0;
    for material in grid_shader_materials.iter_mut() {
        zoom = material.1.default_zoom / material.1.zoom;
    }

    for mut transform in &mut ball_query {
        transform.scale = vec3(zoom, zoom, 1.0);
    }
}

fn move_ball(
    mut ball_query: Query<(&mut Transform, &mut Ball), With<Ball>>,
    mut grid_shader_materials: ResMut<Assets<grid::GridShader>>,
    time: Res<SimulationTiming>,
) {
    // Get variables from grid texture
    let mut zoom = 1.0;
    let mut camera_offset = vec2(0.0, 0.0);
    for material in grid_shader_materials.iter_mut() {
        zoom = material.1.default_zoom / material.1.zoom;
        camera_offset = material.1.camera_offset;
    }

    // Get list of positions for following a ball
    let mut ball_transform_hashmap: HashMap<i32, Vec2> = HashMap::new();
    for (transform, ball) in &mut ball_query {
        ball_transform_hashmap.insert(
            ball.id,
            vec2(transform.translation.x, transform.translation.y),
        );
    }

    // Move Ball
    for (mut transform, ball) in &mut ball_query {
        let mut translation = transform.translation;
        let angle = (2.0 * PI) / ball.time_period * time.time;
        translation.x = ball.radius * zoom * f32::cos(angle) - camera_offset.x;
        translation.y = ball.radius * zoom * f32::sin(angle) + camera_offset.y;

        // Following a ball
        if let Some(id) = ball.follow_id {
            let follow_ball_query = ball_transform_hashmap.iter().find(|b| b.0.to_owned() == id);
            if let Some(follow_ball) = follow_ball_query {
                translation.x += follow_ball.1.x + camera_offset.x; // Offset camera x and y as it's already done above
                translation.y += follow_ball.1.y - camera_offset.y;
            }
        }

        transform.translation = translation;
    }
}
