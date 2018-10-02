#version 140

in vec3 position;

uniform mat4 perspective;
uniform mat4 transform;
uniform int frame;

uniform vec3 origin = vec3(-0.5, -0.5, 0.0); 

void main() {
    vec3 pos = position;
    pos.z += sin(frame * 0.1 + distance(pos, origin)) * 0.1;
    gl_Position = perspective * transform * vec4(pos, 1.0);
}