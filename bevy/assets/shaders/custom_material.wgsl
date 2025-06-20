// 'uv's are in the MeshVertexOutput
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> window_size: vec2<f32>;
@group(2) @binding(1) var<uniform> zoom: f32;
@group(2) @binding(2) var<uniform> camera_offset: vec2<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
	let gap = round(50.0 / zoom);  // Known bug where lines are drawn wrong on some prime numbers
	let real_coords = floor(mesh.uv.xy * window_size) - vec2(0.5) * window_size + camera_offset;
	let m = abs(real_coords % vec2(gap));
	let fragColor = vec4(vec3(1.0) - vec3(1.0) * clamp(min(m.x, m.y), clamp(min(abs(real_coords.x), abs(real_coords.y)), 0.8, 0.98), 1.0), 1.0);
	return fragColor;
}

