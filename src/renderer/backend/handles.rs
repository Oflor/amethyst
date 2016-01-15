use super::{Resources, States};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Buffer<R: Resources>(R::Buffer);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Pipeline<S: States>(S::Pipeline);
