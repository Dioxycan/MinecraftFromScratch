#version 460

layout(location=0)in vec3 position;
layout(location=1)in vec3 color;
layout(location=0)out vec3 fragColor;

layout(push_constant)uniform Push
{
    mat4 proj_view;
}push;

void main(){
    
    gl_Position=push.proj_view*vec4(position,1.0);
    fragColor=color;
}

