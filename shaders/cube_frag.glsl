#version 430

uniform float light_intensity;
uniform samplerCube texture_sampler;

in vec3 tex_dir;

out vec4 color;

void main() {
    color = light_intensity * texture(texture_sampler, tex_dir);
}
