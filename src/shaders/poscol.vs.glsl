#version 140

in vec3 position;

uniform mat4 perspective;
uniform mat4 transform;
uniform int frame;

out vec3 pos;

void main() {
    gl_Position = perspective * transform * vec4(position, 1.0);
    pos = position;
}