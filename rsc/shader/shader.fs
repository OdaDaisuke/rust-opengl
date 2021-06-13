#version 140

in vec3 FragPosition;

void main()
{
    gl_FragColor = vec4(sin(FragPosition.x), cos(FragPosition.y), 0.0, 1.0);
}
