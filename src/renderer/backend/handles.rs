//! Opaque handles to graphics API data.

use super::{Resources, States};

/// A buffer in GPU memory.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Buffer<R: Resources>(R::Buffer);

/// A pipeline state object.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Pipeline<S: States>(S::Pipeline);
