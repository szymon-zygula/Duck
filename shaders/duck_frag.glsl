#version 430

uniform vec3 light_position;
uniform float light_intensity;
uniform sampler2D texture_sampler;

in VS_OUT {
    vec3 position;
    vec3 normal;
    vec2 tex;
} fs_in;

out vec4 color;

void main() {
    color = texture(texture_sampler, fs_in.tex);
}
