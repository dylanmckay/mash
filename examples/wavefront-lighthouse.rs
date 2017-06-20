extern crate mash;

type Vertex = mash::Vector;
type Index = u32;

type Model = mash::Model<Vertex, Index>;

fn main() {
    let wavefront = mash::load::wavefront::from_path("res/lighthouse.obj").unwrap();

    let pylon_obj = wavefront.objects().find(|o| o.name() == "pylon_rectangle").unwrap();
    let lights_obj = wavefront.objects().find(|o| o.name() == "rotating_lights_cylinder").unwrap();

    let pylon = Model::new(pylon_obj).unwrap();
    let lights = Model::new(lights_obj).unwrap();

    print_information("pylon", &pylon);
    print_information("lights", &lights);
}

fn print_information(name: &str, model: &Model) {
    println!("{}", name);
    println!("-----------------------------");
    println!("vertices: {}", model.mesh.vertices.len());
    println!("indices: {}", model.mesh.indices.len());
    println!("triangles: {}", model.mesh.triangles().count());
    println!("");
}

