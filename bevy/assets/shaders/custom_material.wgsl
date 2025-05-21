// 'uv's are in the MeshVertexOutput
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::view View

@group(0) @binding(0) var<uniform> view: View;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
	let gap = 50.0;
	let normalised_uv = (mesh.uv.xy * 2.0) - 1.0;
	let m = abs((normalised_uv * vec2(1280.0, 720.0) * .5) % vec2(gap));
	let fragColor = vec4(vec3(1.0) - vec3(1.0) * clamp(min(m.x, m.y), 0.0, 1.0), 1.0);
	return fragColor;
}

