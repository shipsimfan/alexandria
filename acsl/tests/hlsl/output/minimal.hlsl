struct __t_VOut;

struct __t_VOut {
    float4 position : SV_POSITION;
}

float4 __f_vertex_main();
float4 __f_pixel_main();

float4 __f_vertex_main() : __t_VOut {
    return float4(0.0, 0.0, 0.0, 1.0);
}

float4 __f_pixel_main(__t_VOut _) : SV_TARGET {
    return float4(1.0, 1.0, 1.0, 1.0);
}