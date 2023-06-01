#version 430

uniform vec3 camera_position;
uniform vec3 light_position;
uniform float light_intensity;
uniform sampler2D texture_sampler;
uniform samplerCube skybox_sampler;

in VS_OUT {
    vec3 position;
    vec2 tex;
} fs_in;

out vec4 color;

const float water_level = -2.5;

const float n1 = 1.0;
const float n2 = 1.3333;
const float F0 = 0.14;

float fresnel(vec3 to_observer, vec3 normal) {
    float cos_view = max(dot(to_observer, normal), 0.0);
    return F0 + (1.0 - F0) * pow(1.0 - cos_view, 5.0);
}

vec3 intersect_ray(vec3 p, vec3 dir) {
    float tx = max((1.0 - p.x) / dir.x, (-1.0 - p.x) / dir.x);
    float ty = max((1.0 - p.y) / dir.y, (-1.0 - p.y) / dir.y);
    float tz = max((1.0 - p.z) / dir.z, (-1.0 - p.z) / dir.z);

    float t = min(min(tx, ty), tz);
    return p + t * dir;
}

void main() {
    vec3 normal = texture(texture_sampler, fs_in.tex).xyz * 2.0 - vec3(1.0, 1.0, 1.0);
    vec3 to_light = normalize(light_position - fs_in.position);
    vec3 to_observer = normalize(camera_position - fs_in.position);

    float n;
    if(camera_position.y < water_level) {
        n = n2 / n1;
        normal = -normal;
    }
    else {
        n = n1 / n2;
    }

    vec3 reflected = reflect(-to_observer, normal);

    vec3 refracted = refract(-to_observer, normal, n);
    vec3 local_pos = vec3(fs_in.tex.x * 2.0 - 1.0, 0.0, fs_in.tex.y * 2.0 - 1.0);
    vec3 intersect_reflect = intersect_ray(local_pos, reflected);
    vec3 intersect_refract = intersect_ray(local_pos, refracted);
    float f = fresnel(to_observer, normal);

    vec3 refl_color = texture(skybox_sampler, intersect_reflect).xyz;
    vec3 refr_color = texture(skybox_sampler, intersect_refract).xyz;

    if (refracted == vec3(0.0, 0.0, 0.0)) {
        f = 1.0;
    }

    vec3 light_reflected = reflect(-to_light, normal);
    float specular_dot = max(dot(to_observer, light_reflected), 0.0);
    vec3 specular = light_intensity * 2.0 * pow(specular_dot, 40.0) * vec3(1.0, 1.0, 1.0);

    color = vec4(specular + f * refl_color + (1.0 - f) * refr_color, 1.0);
}
