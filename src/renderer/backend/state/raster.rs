pub struct RasterizerInfo {
    pub cull_mode: CullMode,
    pub depth_bias: i32,
    pub depth_bias_slope_scaled: f32,
    pub depth_bias_clamp: f32,
    pub fill_mode: FillMode,
    pub winding_order: Winding,
}
