struct VOut {
    float position : SV_POSITION;
}

float4 vertex_main() : VOut {
    return float4(0.0, 0.0, 0.0, 1.0);
}

float4 pixel_main(VOut _) : SV_TARGET {
    return float4(1.0, 1.0, 1.0, 1.0);
}