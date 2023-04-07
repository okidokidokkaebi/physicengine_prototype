#version 460

uniform vec3 camera_position;

out vec4 color;

in vec4 fColor;

void main() {
    color = fColor;
}