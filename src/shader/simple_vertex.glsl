#version 460

uniform mat4 model;
uniform mat4 view;

in vec3 position;
in vec3 normal;

void main() {
    gl_Position = view * model * vec4(position, 1.0);
}