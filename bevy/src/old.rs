use bevy::{
    color::palettes::css::{GRAY, GREEN},
    math::ops::powf,
    prelude::*,
    render::{
        camera::Viewport,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::Material2dPlugin,
};

const SHADER_ASSET_PATH: &str = "shaders/custom_material.wgsl";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulator".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
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
) {
    let Ok(_) = window.single() else {
        return;
    };
    let Ok((_, mut transform, mut projection)) = camera_query.single_mut() else {
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
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
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
    // commands.spawn((
    //     Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
    //     MeshMaterial2d(materials.add(Color::from(GREEN))),
    // ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(CustomMaterial {
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behaviour. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
