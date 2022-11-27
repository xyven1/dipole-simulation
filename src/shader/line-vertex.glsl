attribute vec3 position;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;


void main (void) {
  vec4 worldPosition = model * vec4(position, 1.0);

  gl_Position = perspective * view * worldPosition;
}

