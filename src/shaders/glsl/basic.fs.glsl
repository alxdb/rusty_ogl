#version 140

in vec3 col;

out vec4 colour;

void main() {
    colour = vec4(col, 1.0);
}