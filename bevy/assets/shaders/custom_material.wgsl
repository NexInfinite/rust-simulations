// 'uv's are in the MeshVertexOutput
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::view View

@group(0) @binding(0) var<uniform> view: View;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
	let gap = 25.0;
	// let normalised_uv = (mesh.uv.xy * 2.0) - 1.0;
	let normalised_uv = mesh.uv;
	let x = normalised_uv.x - 1080.0 * .5 / gap;
	let y = normalised_uv.y - 720.0 * .5 / gap;
	// let mx = abs((x - trunc(x)) * gap);
	// let my = abs((y - trunc(y)) * gap);

	let m = abs((mesh.uv - view.viewport.xy * .5) % vec2(25.0));

  let fragColor = vec4(vec3(1.0) - vec3(1.0) * clamp(min(m.x, m.y), 0.0, 1.0), 1.0);
	return fragColor;
}