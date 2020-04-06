# mash

[![Build Status](https://travis-ci.com/dylanmckay/mash.svg?token=yjrpKtNnXqa4h6sV1BQh&branch=master)](https://travis-ci.com/dylanmckay/mash)
[![license](https://img.shields.io/github/license/dylanmckay/mash.svg)]()

[Documentation](https://docs.rs/mash)

3D mesh manipulation library.

## Supported formats

All formats are enabled by default when including `mash` as a dependency.

* [Wavefront OBJ](https://en.wikipedia.org/wiki/Wavefront_.obj_file)

In order to pick and choose which formats are supported, explicitly set which features
you want to enable.

```toml
[dependencies]
mash = { version = "1.0", default-features = false, features = ["wavefront"]}
```

## Architecture

Models are first loaded into memory into format-specific structures, to allow the most flexibility.
You will find documentation for these types inside the relevant `load/<format>` module.

Once the relevant objects/groups/triangles are separated out, the models can then converted into a
format-independent representation - `mash::Model`.

The basic workflow looks like this

```rust
extern crate mash;

use mash::load;

type Vertex = mash::Vector;
type Index = u32;
type Model = mash::Model<Vertex, Index>;

fn main() {
    // Load the shape into a wavefront-specific data structure.
    let world = load::wavefront::from_path("res/world.obj").unwrap();

    // Rather than converting the entire world into a single model, let's extract
    // every object labelled 'door'.
    let doors: Vec<Model> = world.objects().filter(|o| o.name().contains("door")).map(|object| {
        // Convert each object into its own mesh.
        Model::new(object).unwrap()
    }).collect();

    // We can also load the entire world into a single model if we wanted.
    let entire_world = Model::new(world).unwrap();

    // Skip every second triangle if that's your kind of thing.
    let half_triangles = entire_world.mesh.triangles().enumerate().filter(|&(idx,_)| idx%2 == 0).map(|(_,t)| t);
    let half_world: Model = Model { mesh: half_triangles.collect() };
}
```

