#version 430 core

// Generated from Alexandria Common Shader Language

layout (location = 0) in vec2 acsl_vertex_input_position;
layout (location = 1) in vec2 acsl_vertex_input_uv;

out vec4 acsl_pixel_input_position;
out vec2 acsl_pixel_input_uv;

struct MatrixBuffer {
    mat4x4 world;
    mat4x4 projection;
    vec4 color;
};

struct VertexInputType {
    vec2 position;
    vec2 uv;
};

struct PixelInputType {
    vec4 position;
    vec2 uv;
};

layout(location = 0) uniform MatrixBuffer matrix_buffer;

layout(location = 32) uniform sampler2D shader_texture;

void main() {
    VertexInputType vertex_input = VertexInputType(acsl_vertex_input_position, acsl_vertex_input_uv);

    PixelInputType acsl_vertex_output = PixelInputType(((vec4(vertex_input.position.x, vertex_input.position.y, 0.0, 1.0) * matrix_buffer.world) * matrix_buffer.projection), vertex_input.uv);
    acsl_pixel_input_position = acsl_vertex_output.position;
    acsl_pixel_input_uv = acsl_vertex_output.uv;
    gl_Position = acsl_pixel_input_position;
    return;
}


