//! Makes low-level graphics API calls and manages memory.

use renderer::ir::CommandBuffer;

mod handles;
mod resources;
mod state;

pub use self::handles::{Buffer, Pipeline};
pub use self::state::PipelineInfo;

/// A trait implemented by renderer backends.
pub trait Backend: Sized {
    type Resources: Resources;
    type States: States;

    fn process(&mut self, buffers: Vec<CommandBuffer>);
} 

/// A trait that describes GPU resources.
pub trait Resources: Clone + Eq + PartialEq {
    type Buffer: Clone + Eq + PartialEq;
    type Fence: Clone + Eq + PartialEq;
    type Sampler: Clone + Eq + PartialEq;
    type Shader: Clone + Eq + PartialEq;
    type Texture: Clone + Eq + PartialEq;
}

/// A trait that describes GPU state objects.
pub trait States: Clone + Eq + PartialEq {
    type Blend: Clone + Eq + PartialEq;
    type DepthStencil: Clone + Eq + PartialEq;
    type Pipeline: Clone + Eq + PartialEq;
    type Raster: Clone + Eq + PartialEq;
    type Viewport: Clone + Eq + PartialEq;
}

pub trait Factory<R: Resources, S: States> {
    fn create_buffer(&mut self) -> Buffer<R::Buffer>;
    fn create_pipeline(&mut self, info: PipelineInfo) -> Pipeline<S::Pipeline>;
}
