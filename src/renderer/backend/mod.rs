//! Makes low-level graphics API calls and manages memory.

use renderer::ir::CommandBuffer;

use std::fmt::Debug;

mod handles;
mod statedyn;
pub mod statestat;

pub use self::handles::{Buffer, Pipeline};

/// Trait implemented by renderer backends.
pub trait Backend: Sized {
    type Resources: Resources;
    type States: States;

    fn process(&mut self, buffers: Vec<CommandBuffer>);
}

pub trait Resources: Clone + Debug + Eq + PartialEq {
    type Buffer: Clone + Debug + Eq + PartialEq;
    type Fence: Clone + Debug + Eq + PartialEq;
    type Sampler: Clone + Debug + Eq + PartialEq;
    type Shader: Clone + Debug + Eq + PartialEq;
    type Texture: Clone + Debug + Eq + PartialEq;
}

pub trait States: Clone + Debug + Eq + PartialEq {
    type Blend: Clone + Debug + Eq + PartialEq;
    type DepthStencil: Clone + Debug + Eq + PartialEq;
    type Pipeline: Clone + Debug + Eq + PartialEq;
    type Raster: Clone + Debug + Eq + PartialEq;
    type Viewport: Clone + Debug + Eq + PartialEq;
}

pub trait Factory<R: Resources, S: States> {
    fn create_buffer(&mut self) -> Buffer<R::Buffer>;
    fn create_pipeline(&mut self, info: PipelineInfo) -> Pipeline<S::Pipeline>;
}
