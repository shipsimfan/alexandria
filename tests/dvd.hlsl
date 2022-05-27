cbuffer MatrixBuffer {
    matrix worldMatrix;
    matrix projectionMatrix;
}

struct VertexInputType {
    float2 position : POSITION;
    float2 uv: TEXCOORD;
};

struct PixelInputType {
    float4 position: SV_POSITION;
    float2 uv: TEXCOORD0;
};

Texture2D shader_texture;
SamplerState sample_type;

PixelInputType vertex_main(VertexInputType input) {
    PixelInputType output;

    output.position = float4(input.position, 0.0, 1.0);
    output.position = mul(output.position, worldMatrix);
    output.position = mul(output.position, projectionMatrix);
    output.uv = input.uv;

    return output;
}

float4 pixel_main(PixelInputType input) : SV_TARGET {
    return shader_texture.Sample(sample_type, input.uv);
}