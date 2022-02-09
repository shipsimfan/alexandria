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
    output.color = input.color;

    return output;
}

float4 pixel_main(PixelInputType input) : SV_TARGET {
    return input.color;
}