#version 430

uniform mat4 model_transform;
uniform mat4 view_transform;
uniform mat4 projection_transform;

layout (location = 0) in vec3 position;

out vec3 tex_dir;

void main() {
    tex_dir = normalize(position - vec3(0.5, 0.5, 0.5));

    gl_Position =
        projection_transform *
        view_transform *
        model_transform *
        vec4(position, 1.0);
}
