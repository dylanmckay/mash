pub mod wavefront;

use {Model, Vertex, Index};

pub trait Format {
    type Vertex;

    fn build_model<V,I>(self) -> Model<V,I>
        where V: Vertex, I: Index, V: From<Self::Vertex>;
}

