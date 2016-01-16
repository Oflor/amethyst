use super::super::Resources;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Buffer<R: Resources> {
    Index(R::Buffer),
    Uniform(R::Buffer),
    Vertex(R::Buffer),
}
