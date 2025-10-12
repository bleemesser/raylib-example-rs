#version 330

// Input vertex attributes (from vertex shader)
in vec2 fragTexCoord;
in vec4 fragColor;

// Output fragment color
out vec4 finalColor;

// Uniforms from the application
uniform vec2 c;
uniform vec2 offset;
uniform float zoom;
uniform vec2 screenSize;

const int maxIterations = 255;
const float colorCycles = 2.0;

// Square a complex number
vec2 ComplexSquare(vec2 z) {
    return vec2(
        z.x * z.x - z.y * z.y,
        2.0 * z.x * z.y
    );
}

// Convert Hue Saturation Value (HSV) color into RGB
vec3 Hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    // --- Aspect Ratio Correction ---
    // Convert fragment coordinates from [0, 1] to [-0.5, 0.5]
    vec2 uv = fragTexCoord - 0.5;
    // Adjust the x-coordinate based on the screen's aspect ratio
    uv.x *= screenSize.x / screenSize.y;

    // Scale and position the view
    vec2 z = uv * 2.5 / zoom;
    z.x += offset.x;
    z.y -= offset.y; // Invert Y-axis for correct panning

    // --- Julia Set Iteration ---
    int iterations = 0;
    for (int i = 0; i < maxIterations; i++) {
        z = ComplexSquare(z) + c;
        if (dot(z, z) > 4.0) {
            break;
        }
        iterations++;
    }

    // --- Color Smoothing ---
    // Perform a few extra iterations to get a more accurate value for smoothing
    // See: http://linas.org/art-gallery/escape/escape.html
    z = ComplexSquare(z) + c;
    z = ComplexSquare(z) + c;
    float smoothVal = float(iterations) + 1.0 - (log(log(length(z))) / log(2.0));

    // Normalize the value to be between 0 and 1
    float norm = smoothVal / float(maxIterations);

    // --- Final Color Calculation ---
    // If the point is in the set (doesn't escape), color it black.
    // Otherwise, map the normalized value to a color from the HSV palette.
    if (iterations == maxIterations) {
        finalColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        finalColor = vec4(Hsv2rgb(vec3(norm * colorCycles, 1.0, 1.0)), 1.0);
    }
}
