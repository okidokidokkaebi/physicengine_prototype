#version 460

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

in vec3 position;
in vec3 normal;

out vec4 mv_normal;
out vec4 mv_position;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    mv_normal = transpose(inverse(view * model)) * vec4(normal, 0.0);
    mv_position = view * model * vec4(position, 1.0);
}