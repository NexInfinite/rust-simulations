use bevy::{
    color::palettes::css::{GREEN, ORANGE, RED},
    prelude::*,
};

use crate::{camera_and_background::grid, resources::timing::SimulationTiming};

use super::selector::SimulationState;

pub struct LinearSimulation;

impl Plugin for LinearSimulation {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Linear), spawn_balls);
        app.add_systems(OnExit(SimulationState::Linear), despawn_balls);
        app.add_systems(
            Update,
            (
                scale_ball.run_if(in_state(SimulationState::Linear)),
                move_ball.run_if(in_state(SimulationState::Linear)),
            ),
        );
    }
}

#[derive(Component, Clone)]
pub struct Ball {
    start_pos: Vec2,
    current_pos: Vec2,
    end_pos: Vec2,
    period: f32,
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Ball {
            start_pos: vec2(0.0, 150.0),
            current_pos: vec2(0.0, 150.0),
            end_pos: vec2(0.0, -150.0),
            period: 5.0,
        },
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Ball {
            start_pos: vec2(150.0, 0.0),
            current_pos: vec2(150.0, 0.0),
            end_pos: vec2(-150.0, 0.0),
            period: 5.0,
        },
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(Color::from(ORANGE))),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Ball {
            start_pos: vec2(150.0, 250.0),
            end_pos: vec2(-150.0, -250.0),
            current_pos: vec2(150.0, 250.0),
            period: 5.0,
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
    simulation_time: Res<SimulationTiming>,
) {
    // Get variables from grid texture (should REALLY find a better way to do this repeatedly)
    let mut zoom = 1.0;
    let mut camera_offset = vec2(0.0, 0.0);
    for material in grid_shader_materials.iter_mut() {
        zoom = material.1.default_zoom / material.1.zoom;
        camera_offset = material.1.camera_offset;
    }

    // Move Ball
    for (mut transform, mut ball) in &mut ball_query {
        if simulation_time.time > 0.0 && !simulation_time.paused {
            ball.current_pos = calculate_new_position(ball.clone(), simulation_time.time);
        }

        // Update position of ball on screen with offsets
        transform.translation = vec3(
            ball.current_pos.x * zoom - camera_offset.x,
            ball.current_pos.y * zoom + camera_offset.y,
            1.0,
        );
    }
}

fn calculate_new_position(ball: Ball, time: f32) -> Vec2 {
    let diff = ball.end_pos - ball.start_pos;
    return ball.start_pos + diff * triangle_wave_position(time, ball.period);
}

fn triangle_wave_position(time: f32, period: f32) -> f32 {
    return 2.0 * f32::abs((time / period) - f32::floor((time / period) + 0.5));
}
