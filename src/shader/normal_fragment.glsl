#version 460

out vec4 color;

void main() {

    color = vec4((m_normal.xyz + 1)/ 2, 1.0);
}