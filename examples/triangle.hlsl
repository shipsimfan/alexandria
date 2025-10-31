struct VOut {
    float4 position : SV_POSITION;
};

VOut vertex_main() {
    VOut vertex;
    vertex.position = float4(0.0, 0.0, 0.0, 1.0);
    return vertex;
}

float4 pixel_main(VOut _) : SV_TARGET {
    return float4(1.0, 1.0, 1.0, 1.0);
}