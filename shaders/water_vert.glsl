#version 430

uniform mat4 model_transform;
uniform mat4 view_transform;
uniform mat4 projection_transform;

layout (location = 0) in vec3 position;

out VS_OUT {
    vec3 position;
    vec2 tex;
} vs_out;

void main() {
    vs_out.position = (model_transform * vec4(position, 1.0)).xyz;

    gl_Position =
        projection_transform *
        view_transform *
        vec4(vs_out.position, 1.0);

    vs_out.tex = position.xz;
}
