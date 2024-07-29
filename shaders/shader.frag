#version 460

out vec4 color;

in vs_Out {
    vec3 normal;
    vec3 fragPos;
    vec2 texCoords;
    vec4 lightSpace;
} fs_in;
uniform vec3 col;
uniform vec3 viewPos;

// point light data and calculations
#define MAX_POINT_LIGHTS 20
uniform struct pointLight {
    vec3 color;
    vec3 position;
} pointLights[MAX_POINT_LIGHTS];
uniform int pointLightCount;
vec3 calc_pointlight(pointLight light);
// texturing object surface
uniform bool textured;
//uniform sampler2D image;
// getting a checkered pattern on an objects surface
uniform bool checkered;
uniform float squares;
float checkered_fn();
// drawing a grid line on an objects surface
uniform bool subDivided;
uniform float lines;
float line_fn();
// blending with background based on distance from camera
// also can be used to create a lazy fog effect
float blend(float far);
// for directional light
uniform vec3 L_direction;
uniform vec3 L_color;
vec3 directional_light();
// for shadow calculations
uniform sampler2D shadowMap;
uniform bool shadowsEnabled;
float ortho_shadow();

void main() {
    vec3 result = vec3(0.0);

    if(textured) {
       // result = texture(image, vs_in.texCoords).rgb;
    } else {
        result += directional_light();

        //for(int i = 0; i < pointLightCount; i++) result += calc_pointlight(pointLights[i]);
    }

    if(checkered) {
        if(checkered_fn() == 1)
            result *= 0.3;
    }
    if(subDivided) {
        if(line_fn() == 1)
            result *= 0.2;
    }

    vec3 background = vec3(0.1);
    float backgroundfract = blend(600.0);
    result = (result * (1.0 - backgroundfract)) + (background * backgroundfract);

    color = vec4(result, 1.0);
    //color = vec4(0.0, 1.0, 0.12, 1.0);
}

// function definations
float blend(
    float far
) {
    float distance = clamp(length(fs_in.fragPos - viewPos), 0.0, far);
    return (pow(distance / far, 2.0));
}

float checkered_fn() {
    float square = 2.0 / squares;

    vec2 value = step(vec2(0.5), fract(fs_in.texCoords / square));
    return int(value.x + value.y) % 2;
}
float line_fn() {
    float line = 1.0 / lines;
    vec2 a = step(vec2(0.005), fract(fs_in.texCoords / line));
    vec2 b = step(vec2(0.005), 1.0 - fract(fs_in.texCoords / line));
    return a.x * a.y * b.x * b.y;
}

vec3 calc_pointlight(pointLight light) {
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
    float spec = pow(max(dot(norm, halfwaydir), 0.0), 32.0);
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
vec3 directional_light() {
    vec3 result = vec3(0.0);

    vec3 ambient = vec3(0.15) * L_color;

    vec3 norm = normalize(fs_in.normal);
    vec3 lightDir = normalize(-L_direction);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * L_color;

    vec3 viewDir = normalize(viewPos - fs_in.fragPos);
    vec3 halfwaydir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(norm, halfwaydir), 0.0), 16.0);
    vec3 specular = spec * L_color;

    if(shadowsEnabled)
        result = (ambient + (1.0 - ortho_shadow()) * (diffuse + specular)) * col;
    else
        result += (ambient + diffuse + specular) * col;

    return result;
}