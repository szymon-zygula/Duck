#version 430

in VS_OUT {
    vec3 position;
    vec3 normal;
    vec2 tex;
} fs_in;

out vec4 color;

void main() {
    vec3 c = fs_in.position + fs_in.normal + vec3(fs_in.tex, 1.0);
    color = vec4(c, 1.0);
}
