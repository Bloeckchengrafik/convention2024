#version 460

layout (location = 0) in vec2 position;
layout (location = 1) in uint32_t material;

void main() {
    if (material == 0) {
        gl_Position = vec4(position, 0.0, 1.0);
    } else {
        gl_Position = vec4(position / 2.0, 0.0, 1.0);
    }
}