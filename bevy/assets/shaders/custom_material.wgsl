// 'uv's are in the MeshVertexOutput
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> window_size: vec2<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
	let gap = 50.0;
	let normalised_uv = (mesh.uv.xy * 2.0) - 1.0;
	let real_coords = normalised_uv * window_size;
	let m = abs((real_coords * .5) % vec2(gap));
	let fragColor = vec4(vec3(1.0) - vec3(1.0) * clamp(min(m.x, m.y), clamp(min(abs(real_coords.x * .5), abs(real_coords.y * .5)), 0.8, 0.95), 1.0), 1.0);
	return fragColor;
}

