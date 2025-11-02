struct VIn {
    float2 position : POSITION;
    float3 color : COLOR;
};

struct VOut {
    float4 position : SV_POSITION;
    float4 color : COLOR0;
};

VOut vertex_main(VIn vin) {
    VOut vout;
    vout.position = float4(vin.position, 0.0, 1.0);
    vout.color = float4(vin.color, 1.0);
    return vout;
}

float4 pixel_main(VOut vout) : SV_TARGET {
    return vout.color;
}