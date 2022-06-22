// Generated from Alexandria Common Shader Language

struct MatrixBuffer {
    float4x4 world;
    float4x4 projection;
    float4 color;
};
MatrixBuffer acsl_create_MatrixBuffer(float4x4 world, float4x4 projection, float4 color) {
    MatrixBuffer output;
    output.world = world;
    output.projection = projection;
    output.color = color;
    return output;
}

struct VertexInputType {
    float2 position: POSITION;
    float2 uv: TEXCOORD;
};
VertexInputType acsl_create_VertexInputType(float2 position, float2 uv) {
    VertexInputType output;
    output.position = position;
    output.uv = uv;
    return output;
}

struct PixelInputType {
    float4 position: SV_POSITION;
    float2 uv: TEXCOORD0;
};
PixelInputType acsl_create_PixelInputType(float4 position, float2 uv) {
    PixelInputType output;
    output.position = position;
    output.uv = uv;
    return output;
}

cbuffer acsl_constant_buffer_0 : register(b0) {
    MatrixBuffer buffer;
}

Texture2D shader_texture : register(t0);
SamplerState acsl_shader_texture_sampler_state : register(s0);

PixelInputType vertex_main(VertexInputType vertex_input) {
    return acsl_create_PixelInputType(mul(mul(float4(vertex_input.position.x, vertex_input.position.y, 0.0, 1.0), buffer.world), buffer.projection), vertex_input.uv);
}

float4 fragment_main(PixelInputType pixel_input) : SV_TARGET {
    return (shader_texture.Sample(acsl_shader_texture_sampler_state, pixel_input.uv) * buffer.color);
}

