#version 460

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 norm;
layout(location = 2) in vec2 tc;
layout(location = 3) in vec3 col;
//layout(location = 3) in vec4 boneWeights;
//layout(location = 4) in ivec4 boneIds;

uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 lightSpace;

out vs_Out {
    vec3 normal;
    vec3 fragCol;
    vec3 fragPos;
    vec2 texCoords;
    vec4 lightSpace;
} vs_out;

void main() {

    vs_out.fragPos = vec3(transform * vec4(pos, 1.0));
    vs_out.normal = mat3(transpose(inverse(transform))) * norm;
    vs_out.texCoords = tc;
    vs_out.fragCol = col;

    vs_out.lightSpace = lightSpace * vec4(vs_out.fragPos, 1.0);

    gl_Position = projection * view * transform * vec4(pos, 1.0);
}
