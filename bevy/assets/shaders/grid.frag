#version 450

// layout(location = 0) in vec2 v_Uv;
// layout(location = 0) out vec4 o_Target;

layout(location = 0) in vec2 v_Uv;
out vec4 gl_FragColor;

void main() {
    // vec2 m = abs((v_Uv - vec2(720, 1280) * .5) % vec2(25));
    // o_Target = vec4(vec3(1.0) - vec3(1.0) * clamp(min(m.x, m.y), 0.0, 1.0), 1.0);
    // o_Target = vec4(v_Uv.x, 0.0, 0.0, 1.0);
    gl_FragColor = vec4(v_Uv.x, 0.0, 0.0, 1.0);
}