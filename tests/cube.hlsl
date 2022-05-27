cbuffer MatrixBuffer {
    matrix worldMatrix;
    matrix viewMatrix;
    matrix projectionMatrix;
}

struct VertexInputType {
    float3 position : POSITION;
    float4 color: COLOR;
};

struct PixelInputType {
    float4 position: SV_POSITION;
    float4 color: COLOR;
};

PixelInputType vertex_main(VertexInputType input) {
    PixelInputType output;

    output.position = float4(input.position, 1.0);
    output.position = mul(output.position, worldMatrix);
    output.position = mul(output.position, viewMatrix);
    output.position = mul(output.position, projectionMatrix);
    output.color = input.color;

    return output;
}

float4 pixel_main(PixelInputType input) : SV_TARGET {
    return input.color;
}