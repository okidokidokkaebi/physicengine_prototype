#version 460

uniform vec3 camera_position;

out vec4 color;

in vec4 mv_position;
in vec4 mv_normal;

void main() {
    color = vec4(1.0) * dot(normalize(camera_position - mv_position.xyz), normalize(mv_normal.xyz));
}