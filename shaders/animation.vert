#version 460

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 norm;
layout(location = 2) in vec2 tc;
layout(location = 3) in vec3 col;
layout(location = 4) in vec4 weights;
layout(location = 5) in ivec4 boneIds;

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

const int MAX_BONES = 200;
const int MAX_BONE_INFLUENCE = 4;
uniform mat4 finalBonesMatrices[MAX_BONES];

void main() {

    mat4 BoneTransform = finalBonesMatrices[boneIds[0]] * weights[0];
    BoneTransform += finalBonesMatrices[boneIds[1]] * weights[1];
    BoneTransform += finalBonesMatrices[boneIds[2]] * weights[2];
    BoneTransform += finalBonesMatrices[boneIds[3]] * weights[3];

    mat4 viewModel = view * transform;
    gl_Position = projection * viewModel * BoneTransform * vec4(pos, 1.0);

    vs_out.normal = mat3(transpose(inverse(transform))) * norm;
    vs_out.fragCol = col;
    vs_out.texCoords = tc;

    vs_out.fragPos = vec3(transform * vec4(pos, 1.0));
   // vs_out.lightSpace=lightSpace;

}
