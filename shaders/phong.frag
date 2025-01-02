// blinn phong fragment shader

#version 460

out vec4 color;

in vs_Out {
    vec3 normal;
    vec3 fragCol;
    vec3 fragPos;
    vec2 texCoords;
    vec4 lightSpace;
} fs_in;
uniform vec3 viewPos;

// point light data and calculations
#define MAX_POINT_LIGHTS 20
uniform struct pointLight {
    vec3 color;
    vec3 position;
} pointLights[MAX_POINT_LIGHTS];
uniform int pointLightCount;
vec3 calc_pointlight(pointLight, vec3);

// for directional light
uniform vec3 L_direction;
uniform vec3 L_color;
vec3 directional_light(vec3);

// object material
uniform vec3 baseColor;
uniform float specular_strength;
uniform bool textured;
uniform bool hasDiffuseTex;
uniform bool hasSpecularTex;
uniform sampler2D diffuse_tex;
uniform sampler2D specular_tex;
uniform sampler2D albedo;

// blending with background based on distance from camera
// also can be used to create a lazy fog effect
float blend(float far);

// for shadow calculations
uniform sampler2D shadowMap;
uniform bool shadowsEnabled;
float ortho_shadow();

//*********** main function entry ************/
void main() {
    vec3 result = vec3(0.0);
    vec3 tex = vec3(texture(albedo, fs_in.texCoords));
    vec3 col = vec3(0.0);

    if(textured) {

        col = tex;

    } else {

        col = fs_in.fragCol;

    }

    col = pow(col, vec3(1.0 / 2.2));

    result += directional_light(col);

   /*  for(int i = 0; i < pointLightCount; i++) {
        result += calc_pointlight(pointLights[i]);
    } */

    //foggy effect 
    vec3 background = vec3(0.1);
    float factor = blend(400.0);
    result = mix(result, background, factor);

    color = vec4(result, 1.0);
    //color = vec4(0.37, 0.74, 1.0, 1.0);
}

float blend(float far) {

    float distance = clamp(length(fs_in.fragPos - viewPos), 0.0, far);
    return (pow(distance / far, 2.0));
}

vec3 calc_pointlight(pointLight light, vec3 col) {
    vec3 result = vec3(0.0);

    vec3 ambient = vec3(0.05 * light.color);
    result += ambient;

    vec3 norm = normalize(fs_in.normal);
    vec3 lightDir = normalize(light.position - fs_in.fragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * light.color;
    result += diffuse;

    vec3 viewDir = normalize(viewPos - fs_in.fragPos);
    vec3 halfwaydir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(norm, halfwaydir), 0.0), specular_strength);
    vec3 specular = spec * light.color;
    result += specular;

    float attenuation = clamp(length(fs_in.fragPos - light.position), 0.0, 200.0) / 200.0;
    result *= (1.0 - pow(attenuation, 2.0)) * col;

    return result;
}

float ortho_shadow() {

    vec3 proojCoords = fs_in.lightSpace.xyz / fs_in.lightSpace.w;
    proojCoords = proojCoords * 0.5 + 0.5;
    float closestDepth = texture(shadowMap, proojCoords.xy).r;
    float currentDepth = proojCoords.z;

    float bias = max(0.0025 * (1.0 - dot(fs_in.normal, L_direction)), 0.00025);

    float shadow = currentDepth - bias > closestDepth ? 1.0 : 0.0;

    if(currentDepth > 1.0)
        shadow = 0.0;

    return shadow;
}

vec3 directional_light(vec3 col) {

    vec3 result = vec3(0.0);

    vec3 ambient = vec3(0.15) * L_color * col;

    vec3 norm = normalize(fs_in.normal);
    vec3 lightDir = normalize(-L_direction);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * L_color * col;

    vec3 viewDir = normalize(viewPos - fs_in.fragPos);
    vec3 halfwaydir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(norm, halfwaydir), 0.0), 32.0);
    vec3 specular = spec * L_color * col;

    if(shadowsEnabled)
        result = ambient + (1.0 - ortho_shadow()) * (diffuse + specular);
    else
        result += ambient + diffuse + specular;

    return result;
}
