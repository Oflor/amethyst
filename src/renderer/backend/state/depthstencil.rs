pub struct DepthStencilInfo {
    pub depth_func: CompareFunc,
    pub depth_enabled: bool,
    pub depth_write_enabled: bool,
    pub depth_bounds_enabled: bool,
    pub max_depth: f32,
    pub min_depth: f32,
    pub stencil_enabled: bool,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub back: DepthStencilOp,
    pub front: DepthStencilOp,
}
