//! Builds IR command buffers from Frames and feeds them into the backend.

use renderer::ir::CommandBuffer;

mod frontend;
pub use self::frontend::{Frontend, Frame};

/// A trait that describes a renderable Frame element.
pub trait Renderable {
    fn to_cmdbuf(&self) -> CommandBuffer;
}
