#version 460

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform vec3 camera_position;

in vec3 position;
in vec3 normal;

out vec4 fColor;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    vec4 m_normal = transpose(inverse(model)) * vec4(normal, 0.0);
    vec4 m_position = model * vec4(position, 1.0);

    vec4 ambient = vec4(0.3, 0.1, 0.6, 1.0);
    vec4 diffuse = vec4(1.0, 1.0, 1.0, 1.0) * max(dot(normalize(camera_position - m_position.xyz), normalize(m_normal.xyz)), 0.0);

    fColor = ambient + diffuse;
}