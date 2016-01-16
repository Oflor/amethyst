pub struct BlendInfo {
    pub blend_constants: [f32; 4],
    pub targets: Vec<TargetBlendInfo>,
}

pub struct TargetBlendInfo {
    pub blending_enabled: bool,
    pub alpha_func: BlendFunc,
    pub color_func: BlendFunc,
    pub dest_alpha: Blend,
    pub dest_color: Blend,
    pub source_alpha: Blend,
    pub source_color: Blend,
}
