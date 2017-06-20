extern crate mash;

type Vertex = mash::Vector;
type Index = u32;

type Model = mash::Model<Vertex, Index>;

/// All information needed to render a door.
pub struct Door {
    name: String,
    ambient_color: mash::Color,
    model: Model,
}

fn main() {
    let wavefront = mash::load::wavefront::from_path("res/world.obj").unwrap();

    // Collect all doors from the model.
    let doors: Vec<_> = wavefront.objects().filter_map(|obj| {
        if obj.name().contains("door") {
            let ambient_color = {
                let material = obj.material().expect("object has no material");
                material.ambient_color()
            };

            Some(Door {
                ambient_color: ambient_color,
                name: obj.name().to_owned(),
                model: Model::new(obj).unwrap(),
            })
        } else {
            None
        }
    }).collect();

    for door in doors {
        println!("found door named '{}' with ambient color {:?} and {} triangles",
                 door.name, door.ambient_color, door.model.mesh.triangles().count());
    }
}

