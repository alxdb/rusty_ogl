#version 140

in vec3 position;
in vec3 colour;

out vec3 col;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 transform;
uniform int frame;

void main() {
    col = colour;
    gl_Position = projection * view * transform * vec4(position, 1.0);
}