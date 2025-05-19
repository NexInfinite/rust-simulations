use bevy::{
    color::palettes::css::{DARK_GRAY, GREEN},
    math::ops::powf,
    prelude::*,
    render::camera::Viewport,
    transform,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulator".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, controls)
        .run();
}

fn controls(
    mut camera_query: Query<(&mut Camera, &mut Transform, &mut Projection)>,
    window: Query<&Window>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
    mut exit: EventWriter<AppExit>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(window) = window.single() else {
        return;
    };
    let Ok((mut camera, mut transform, mut projection)) = camera_query.single_mut() else {
        return;
    };
    let f32_speed = 400.0 * time.delta_secs();

    // Exiting
    if input.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }

    // Camera movement
    if input.pressed(KeyCode::KeyS) {
        transform.translation.y += f32_speed;
    }
    if input.pressed(KeyCode::KeyW) {
        transform.translation.y -= f32_speed;
    }
    if input.pressed(KeyCode::KeyD) {
        transform.translation.x -= f32_speed;
    }
    if input.pressed(KeyCode::KeyA) {
        transform.translation.x += f32_speed;
    }

    // Camera zoom controls
    if let Projection::Orthographic(projection2d) = &mut *projection {
        if input.pressed(KeyCode::Comma) {
            projection2d.scale *= powf(4.0, time.delta_secs());
        }

        if input.pressed(KeyCode::Period) {
            projection2d.scale *= powf(0.25, time.delta_secs());
        }

        // XY White Lines
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(
                1.0,
                (window.height() + f32::abs(transform.translation.y) * 2.0) * projection2d.scale,
            ))),
            MeshMaterial2d(materials.add(Color::srgb(0.75, 0.75, 0.75))),
            Transform::from_translation(Vec3::new(0.0, 0.0, -1000.0)),
        ));
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(
                (window.width() + f32::abs(transform.translation.x) * 2.0) * projection2d.scale,
                1.0,
            ))),
            MeshMaterial2d(materials.add(Color::srgb(0.75, 0.75, 0.75))),
            Transform::from_translation(Vec3::new(0.0, 0.0, -1000.0)),
        ));

        // Gray lines every 500 pixels
        let step = 100;
        for i in (step..(window.width() / 2.0 * projection2d.scale) as i32).step_by(step as usize) {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(
                    1.0,
                    (window.height() + f32::abs(transform.translation.y) * 2.0)
                        * projection2d.scale,
                ))),
                MeshMaterial2d(materials.add(Color::srgb(0.25, 0.25, 0.25))),
                Transform::from_translation(Vec3::new(i as f32, 0.0, -1000.0)),
            ));
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(
                    1.0,
                    (window.height() + f32::abs(transform.translation.y) * 2.0)
                        * projection2d.scale,
                ))),
                MeshMaterial2d(materials.add(Color::srgb(0.25, 0.25, 0.25))),
                Transform::from_translation(Vec3::new(-i as f32, 0.0, -1000.0)),
            ));
        }

        for i in (step..(window.height() / 2.0 * projection2d.scale) as i32).step_by(step as usize)
        {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(
                    (window.width() + f32::abs(transform.translation.x) * 2.0) * projection2d.scale,
                    1.0,
                ))),
                MeshMaterial2d(materials.add(Color::srgb(0.25, 0.25, 0.25))),
                Transform::from_translation(Vec3::new(i as f32, 0.0, -1000.0)),
            ));
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let window_size = window.resolution.physical_size().as_vec2();

    // Initialize centered, non-window-filling viewport
    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 { x: 0, y: 0 },
                physical_size: window_size.as_uvec2(),
                ..default()
            }),
            ..default()
        },
    ));

    // Create a minimal UI explaining how to interact with the example
    commands.spawn((
        Text::new(
            "Normal 'gaming' controls to move around,\nuse the comma and period keys to zoom in and out.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    // Add mesh to make camera movement visible
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
    ));
}
