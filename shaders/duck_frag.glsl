#version 430

uniform vec3 camera_position;
uniform vec3 light_position;
uniform float light_intensity;
uniform sampler2D texture_sampler;

in VS_OUT {
    vec3 position;
    vec3 normal;
    vec3 tangent;
    vec2 tex;
} fs_in;

out vec4 color;

const float ambient = 0.3;
const float diffuse_coeff = 0.8;
const float specular_coeff = 0.5;
const float specular_exp = 40.0;

void main() {
    vec3 to_light = normalize(light_position - fs_in.position);
    vec3 to_observer = normalize(camera_position - fs_in.position);
    vec3 reflected = 2.0 * dot(fs_in.normal, to_light) * fs_in.normal - to_light;
    vec3 halff = normalize(to_light + to_observer);

    float diffuse = diffuse_coeff * max(0.0, dot(fs_in.normal, to_light));
    float dot_h_t = dot(halff, fs_in.tangent);
    float specular_cos = max(0.0, sqrt(1.0 - dot_h_t * dot_h_t));
    float specular = specular_coeff * pow(specular_cos, specular_exp);

    color = texture(texture_sampler, fs_in.tex) * light_intensity * (ambient + diffuse + specular);
}
