use {TriangularMesh, Vertex, Index, Vector};

/// Creates a cube with a side length of one.
pub fn unit_cube<V,I>() -> TriangularMesh<V,I>
    where V: Vertex, I: Index, V: From<Vector> {
    self::cube(1.0)
}

/// Create a cube.
///
/// * `scale` represents the side length.
pub fn cube<V,I>(scale: f32) -> TriangularMesh<V,I>
    where V: Vertex, I: Index, V: From<Vector> {
    let vertices = vec![
        Vector(scale, -scale, -scale),
        Vector(scale, -scale, scale),
        Vector(-scale, -scale, scale),
        Vector(-scale, -scale, -scale),
        Vector(scale,  scale, -scale),
        Vector(scale,  scale,  scale),
        Vector(-scale,  scale,  scale),
        Vector(-scale,  scale, -scale),
    ].into_iter().map(Into::into).collect();

    let indices: Vec<I> = [
        1, 3, 0,
        7, 5, 4,
        4, 1, 0,
        5, 2, 1,
        2, 7, 3,
        0, 7, 4,
        1, 2, 3,
        7, 6, 5,
        4, 5, 1,
        5, 6, 2,
        2, 6, 7,
        0, 3, 7,
    ].iter().cloned().map(Into::into).collect();

    TriangularMesh {
        vertices: vertices,
        indices: indices,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_build_cube() {
        let cube: TriangularMesh<Vector, u64> = unit_cube();
        assert_eq!(cube.triangles().count(), 12);
    }
}

