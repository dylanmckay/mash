#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector(pub f32, pub f32, pub f32);

pub trait Vertex {
    fn position(&self) -> Vector;
}

