struct VIn {
    float2 position : POSITION;
};

struct VOut {
    float4 position : SV_POSITION;
};

VOut vertex_main(VIn vin) {
    VOut vout;
    vout.position = float4(vin.position, 0.0, 1.0);
    return vout;
}

float4 pixel_main(VOut _) : SV_TARGET {
    return float4(1.0, 1.0, 1.0, 1.0);
}