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
    vec3 tangent;
    vec2 tex;
} vs_out;

vec3 tangent1() {
    vec3 v = normal;
    vec3 axis = normalize(vec3(0.0, v.z, -v.y));
    return cross(axis, v) + dot(axis, v) * axis;
}

vec3 tangent2() {
    vec3 v = normal;
    vec3 axis = normalize(vec3(tex.y * 2.0 - 1.0, 0.0, 0.0));
    return cross(axis, v) + dot(axis, v) * axis;
}

void main() {
    vs_out.position = (model_transform * vec4(position, 1.0f)).xyz;
    vec3 tangent = tangent1();

    mat4 vec_transform = transpose(inverse(model_transform));
    vs_out.normal = normalize((vec_transform * vec4(normal, 0.0)).xyz);
    vs_out.tangent = normalize((vec_transform * vec4(tangent, 0.0)).xyz);

    vs_out.tex = tex;

    gl_Position =
        projection_transform *
        view_transform *
        vec4(vs_out.position, 1.0);
}
