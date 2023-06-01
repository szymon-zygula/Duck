#version 430

uniform mat4 model_transform;
uniform mat4 view_transform;
uniform mat4 projection_transform;

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex;

out VS_OUT {
    vec3 position;
    vec3 normal;
    vec2 tex;
} vs_out;

void main() {
    vs_out.position = (model_transform * vec4(position, 1.0f)).xyz;

    vs_out.normal = normalize((transpose(inverse(model_transform)) * vec4(normal, 0.0)).xyz);
    vs_out.tex = tex;

    gl_Position =
        projection_transform *
        view_transform *
        vec4(vs_out.position, 1.0);
}
