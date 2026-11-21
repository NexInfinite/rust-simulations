// 'uv's are in the MeshVertexOutput
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> window_size: vec2<f32>;
@group(2) @binding(1) var<uniform> zoom: f32;
@group(2) @binding(2) var<uniform> camera_offset: vec2<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
	return vec4(0.0, 0.0, 0.0, 1.0);
}

