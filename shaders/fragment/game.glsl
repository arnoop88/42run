#version 330 core
out vec4 FragColor;

in vec3 Color;
in vec2 TexCoords;

uniform sampler2D texture_diffuse;

void main() {
    vec4 texColor = texture(texture_diffuse, TexCoords);
    FragColor = texColor;
}