#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

out vec4 finalColor;

uniform vec2 c;
uniform vec2 offset;
uniform float zoom;
uniform vec2 screenSize;

const int maxIterations = 255;
const float colorCycles = 2.0;

vec2 ComplexSquare(vec2 z) {
    return vec2(
        z.x * z.x - z.y * z.y,
        2.0 * z.x * z.y
    );
}

vec3 Hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    vec2 uv = fragTexCoord - 0.5;
    uv.x *= screenSize.x / screenSize.y;

    vec2 z = uv * 2.5 / zoom;
    z.x += offset.x;
    z.y -= offset.y;

    int iterations = 0;
    for (int i = 0; i < maxIterations; i++) {
        z = ComplexSquare(z) + c;
        if (dot(z, z) > 4.0) {
            break;
        }
        iterations++;
    }

    z = ComplexSquare(z) + c;
    z = ComplexSquare(z) + c;
    float smoothVal = float(iterations) + 1.0 - (log(log(length(z))) / log(2.0));

    float norm = smoothVal / float(maxIterations);

    if (iterations == maxIterations) {
        finalColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        finalColor = vec4(Hsv2rgb(vec3(norm * colorCycles, 1.0, 1.0)), 1.0);
    }
}
