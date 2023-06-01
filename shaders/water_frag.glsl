#version 430

uniform vec3 camera_position;
uniform vec3 light_position;
uniform float light_intensity;
uniform sampler2D texture_sampler;

in VS_OUT {
    vec3 position;
    vec2 tex;
} fs_in;

out vec4 color;

const float ambient = 0.3;
const float diffuse_coeff = 0.6;
const float specular_coeff = 1.5;
const float specular_exp = 10.0;
const vec4 water_color = vec4(0.1, 0.5, 1.0, 1.0);

void main() {
    vec3 normal = texture(texture_sampler, fs_in.tex).xyz * 2.0 - vec3(1.0, 1.0, 1.0);
    vec3 to_light = normalize(light_position - fs_in.position);
    vec3 to_observer = normalize(camera_position - fs_in.position);
    vec3 reflected = 2.0 * dot(normal, to_light) * normal - to_light;

    float diffuse = diffuse_coeff * max(0.0, dot(normal, to_light));
    float specular = specular_coeff * pow(max(0.0, dot(to_observer, reflected)), specular_exp);

    color = water_color * light_intensity * (ambient + diffuse + specular);
}
