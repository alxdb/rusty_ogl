#version 140

in vec3 pos;

out vec4 colour;

uniform vec3 origin = vec3(0.0, 0.0, 0.0);

void main() {
    colour = vec4(abs(pos.x - origin.x), abs(pos.y - origin.y), abs(pos.z - origin.z), 1.0);
}