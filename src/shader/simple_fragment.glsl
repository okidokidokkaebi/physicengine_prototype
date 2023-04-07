#version 460

uniform vec3 camera_position;

out vec4 color;

in vec4 m_normal;
in vec4 m_position;

void main() {
    vec4 ambient = vec4(0.3, 0.1, 0.6, 1.0);
    vec4 diffuse = vec4(1.0, 1.0, 1.0, 1.0) * max(dot(normalize(camera_position - m_position.xyz/m_position.w), normalize(m_normal.xyz)), 0.0);

    color = vec4((m_normal.xyz + 1)/ 2, 1.0);
}