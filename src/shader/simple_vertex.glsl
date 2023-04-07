#version 460

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform vec3 camera_position;

in vec3 position;
in vec3 normal;

out vec4 m_normal;
out vec4 m_position;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    m_normal = transpose(inverse(model)) * vec4(normal, 0.0);
    m_position = model * vec4(position, 1.0);
}