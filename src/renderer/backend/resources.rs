use std::fmt::Debug;

pub trait Resources: Clone + Debug + Eq + PartialEq {
    type Buffer: Clone + Debug + Eq + PartialEq;
    type Fence: Clone + Debug + Eq + PartialEq;
    type Pipeline: Clone + Debug + Eq + PartialEq;
    type Sampler: Clone + Debug + Eq + PartialEq;
    type Shader: Clone + Debug + Eq + PartialEq;
    type Texture: Clone + Debug + Eq + PartialEq;
}

pub trait Factory<R: Resources> {
    fn create_buffer(&mut self);
}
