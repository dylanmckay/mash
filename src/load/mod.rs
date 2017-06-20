#[cfg(feature = "wavefront")]
pub mod wavefront;

use {Model, Vertex, Index, Error};

pub trait Format {
    type Vertex;

    fn build_model<V,I>(self) -> Result<Model<V,I>, Error>
        where V: Vertex, I: Index, V: From<Self::Vertex>;
}

