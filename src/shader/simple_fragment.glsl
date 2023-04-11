#version 460

uniform vec3 diffuse_input; 
uniform vec3 camera_position;

in vec4 m_normal;
in vec4 m_position;

out vec4 color;

void main() {
    vec4 ambient = vec4(0.1, 0.0, 0.0, 1.0);
    vec4 diffuse = vec4(diffuse_input
        * ( max(dot(normalize(camera_position - m_position.xyz/m_position.w), normalize(m_normal.xyz)), 0.0)), 1.0);

    color = ambient + diffuse;
}