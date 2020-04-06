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

    for door in doors {
        println!("door model: {:?}", door);
    }

    // We can also load the entire world into a single model if we wanted.
    let entire_world = Model::new(world).unwrap();

    // Skip every second triangle if that's your kind of thing.
    let half_triangles = entire_world.mesh.triangles().enumerate().filter(|&(idx,_)| idx%2 == 0).map(|(_,t)| t);
    let half_world: Model = Model { mesh: half_triangles.collect() };

    println!("half world: {:?}", half_world);
}

