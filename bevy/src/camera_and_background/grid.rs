use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    math::ops::powf,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/custom_material.wgsl";
const DEFAULT_ZOOM: f32 = 0.5;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<GridShader>::default());
        app.add_systems(Startup, camera_setup);
        app.add_systems(Update, scale_background);
        app.add_systems(Update, controls);
    }
}

// Setup a simple 2d scene
fn camera_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridShader>>,
    window: Single<&Window>,
) {
    let window_size = window.resolution.physical_size().as_vec2();
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(GridShader {
            window_size: window_size,
            camera_offset: vec2(0.0, 0.0),
            default_zoom: DEFAULT_ZOOM, // Must be the same as zoom
            zoom: DEFAULT_ZOOM,         // 0-1 is zooming in, >1 is zooming out (ikr)
        })),
        Transform::from_scale(vec3(window_size.x, window_size.y, 0.0)),
        GridMesh,
    ));
}

fn controls(
    mut materials: ResMut<Assets<GridShader>>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut evr_motion: EventReader<MouseMotion>,
    mouse: Res<ButtonInput<MouseButton>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    for material in materials.iter_mut() {
        // Scroll Zoom
        for ev in evr_scroll.read() {
            if ev.unit == MouseScrollUnit::Line && ev.y < 0.0 {
                material.1.zoom =
                    f32::clamp(material.1.zoom * powf(16.0, time.delta_secs()), 0.25, 4.0);
            }

            if ev.unit == MouseScrollUnit::Line && ev.y > 0.0 {
                material.1.zoom = f32::clamp(
                    material.1.zoom * powf(1.0 / 16.0, time.delta_secs()),
                    0.25,
                    4.0,
                );
            }
        }

        // Input Zoom
        if input.pressed(KeyCode::ShiftLeft) {
            material.1.zoom = f32::clamp(material.1.zoom * powf(4.0, time.delta_secs()), 0.25, 4.0);
        }
        if input.pressed(KeyCode::ControlLeft) {
            material.1.zoom =
                f32::clamp(material.1.zoom * powf(0.25, time.delta_secs()), 0.25, 4.0);
        }

        // Move "camera" offset
        let impulse = 400.0;
        if input.pressed(KeyCode::KeyD) {
            material.1.camera_offset.x += impulse * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyA) {
            material.1.camera_offset.x -= impulse * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyW) {
            material.1.camera_offset.y -= impulse * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyS) {
            material.1.camera_offset.y += impulse * time.delta_secs();
        }

        // Mouse panning
        for ev in evr_motion.read() {
            if mouse.pressed(MouseButton::Left) {
                material.1.camera_offset -=
                    ev.delta * vec2(time.delta_secs(), time.delta_secs()) * vec2(50.0, 50.0);
            }
        }

        // Reset everything
        if input.pressed(KeyCode::KeyO) {
            material.1.camera_offset = vec2(0.0, 0.0);
            material.1.zoom = DEFAULT_ZOOM;
        }
    }
}

fn scale_background(
    mut query: Query<&mut Transform, With<GridMesh>>,
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
    pub zoom: f32,
    #[uniform(2)]
    pub camera_offset: Vec2,
    pub default_zoom: f32,
}

#[derive(TypePath, Component)]
pub struct GridMesh;

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
