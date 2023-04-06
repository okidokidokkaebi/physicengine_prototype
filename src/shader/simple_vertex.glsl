#version 460

in vec3 position;
in vec3 normal;

void main() {
    gl_Position = vec4(position.x, position.y, position.z, 1.0);
}