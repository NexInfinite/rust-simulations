use bevy::{
    math::ops::powf,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/custom_material.wgsl";

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<GridShader>::default());
        app.add_systems(Startup, camera_setup);
        app.add_systems(Update, scale_background);
        app.add_systems(Update, camera_controls);
    }
}

// Setup a simple 2d scene
pub fn camera_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridShader>>,
    window: Single<&Window>,
) {
    // Camera and window
    commands.spawn((Camera2d, CustomCamera {}));

    // Grid
    let window_size = window.resolution.physical_size().as_vec2();
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(GridShader {
            window_size: window_size,
            zoom: 1.0,
        })),
        Transform::from_scale(vec3(window_size.x, window_size.y, 0.0)),
    ));
}

pub fn camera_controls(
    mut materials: ResMut<Assets<GridShader>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
    mut exit: EventWriter<AppExit>,
) {
    // Exiting
    if input.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }

    // Zoom
    for material in materials.iter_mut() {
        if input.pressed(KeyCode::Space) {
            material.1.zoom = f32::clamp(material.1.zoom * powf(4.0, time.delta_secs()), 0.25, 4.0);
        }

        if input.pressed(KeyCode::ControlLeft) {
            material.1.zoom =
                f32::clamp(material.1.zoom * powf(0.25, time.delta_secs()), 0.25, 4.0);
        }
    }
}

pub fn scale_background(
    mut query: Query<&mut Transform, With<Mesh2d>>,
    mut materials: ResMut<Assets<GridShader>>,
    window: Single<&Window>,
) {
    let window_size = window.resolution.physical_size().as_vec2();
    for mut transform in &mut query {
        transform.scale = vec3(window_size.x, window_size.y, 0.0);
    }
    for material in materials.iter_mut() {
        material.1.window_size = window_size;
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridShader {
    #[uniform(0)]
    window_size: Vec2,
    #[uniform(1)]
    zoom: f32,
}

#[derive(TypePath, Component)]
pub struct CustomCamera {}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behaviour. See the Material2d api docs for details!
impl Material2d for GridShader {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
