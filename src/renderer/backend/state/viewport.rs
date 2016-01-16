pub struct ViewportInfo {
    pub scissor_test_enabled: bool,
    pub scissors: Vec<ScissorBox>,
    pub viewports: Vec<Viewport>,
}
