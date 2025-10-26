struct __t_VOut;

struct __t_VOut {
    float4 position : SV_POSITION;
}

__t_VOut __f_vertex_main();
float4 __f_pixel_main() : SV_TARGET;

__t_VOut __f_vertex_main() {
    __t_VOut __i_0;
    __i_0.position = float4(0.0, 0.0, 0.0, 1.0);
    return __i_0;
}

float4 __f_pixel_main(__t_VOut _) : SV_TARGET {
    return float4(1.0, 1.0, 1.0, 1.0);
}