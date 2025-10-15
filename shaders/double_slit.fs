#version 330

in vec2 fragTexCoord;
out vec4 finalColor;

// Uniforms from the main application
uniform vec2 screenSize;
uniform float time;
uniform float wavelength;
uniform float slitWidth;      // pixels   
uniform float slitDistance;   // pixels
uniform float wallXPos;       // normalized -0.5 to 0.5
uniform float wallThickness;  // pixels 

const float PI = 3.14159265359;
const float SPEED = 3.0;

void main() {
    // Get pos in normalized coordinates (-0.5 to 0.5 centered), with correct aspect ratio
    vec2 pos = (fragTexCoord - 0.5) * vec2(screenSize.x / screenSize.y, 1.0);

    float slitWidthNorm = slitWidth / screenSize.y;
    float slitDistanceNorm = slitDistance / screenSize.y;
    float wallThicknessNorm = wallThickness / screenSize.x;
    
    float wallLeft = wallXPos - wallThicknessNorm / 2.0;
    float wallRight = wallXPos + wallThicknessNorm / 2.0;

    if (pos.x < wallLeft) {
        float wave = 0.5 + 0.5 * sin(pos.x * (2.0 * PI / wavelength) - time * SPEED);
        finalColor = vec4(vec3(wave), 1.0);
        return;
    }

    float slit1Y_top = slitDistanceNorm / 2.0 + slitWidthNorm / 2.0;
    float slit1Y_bottom = slitDistanceNorm / 2.0 - slitWidthNorm / 2.0;
    float slit2Y_top = -slitDistanceNorm / 2.0 + slitWidthNorm / 2.0;
    float slit2Y_bottom = -slitDistanceNorm / 2.0 - slitWidthNorm / 2.0;

    bool inSlit1 = (pos.y < slit1Y_top && pos.y > slit1Y_bottom);
    bool inSlit2 = (pos.y < slit2Y_top && pos.y > slit2Y_bottom);

    if (pos.x >= wallLeft && pos.x <= wallRight) {
        if (inSlit1 || inSlit2) {
        } else {
            finalColor = vec4(0.0, 0.0, 0.0, 1.0);
            return;
        }
    }

    float totalWave = 0.0;
    int numSamples = 255;
    float amplitudeScale = sqrt(slitWidthNorm / float(numSamples));

    for (int i = 0; i < numSamples; i++) {
        float t = float(i) / float(numSamples - 1);

        float y1 = mix(slit1Y_bottom, slit1Y_top, t);
        vec2 p1 = vec2(wallXPos, y1);
        float r1 = length(pos - p1);
        if (r1 > 0.0) {
           totalWave += (amplitudeScale / sqrt(r1)) * sin((2.0 * PI / wavelength) * r1 - time * SPEED);
        }

        float y2 = mix(slit2Y_bottom, slit2Y_top, t);
        vec2 p2 = vec2(wallXPos, y2);
        float r2 = length(pos - p2);
        if (r2 > 0.0) {
            totalWave += (amplitudeScale / sqrt(r2)) * sin((2.0 * PI / wavelength) * r2 - time * SPEED);
        }
    }

    totalWave *= 0.25; // overall scaling to keep intensity in range
    float finalIntensity = 0.5 + 0.5 * totalWave;
    finalColor = vec4(vec3(finalIntensity), 1.0);
}