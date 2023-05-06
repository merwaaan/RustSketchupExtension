use crate::ruby::{rb_ary_new_capa, rb_int2inum, Value};
use mesh_generation::{generate_noise_map, PolyMesh};
use noise::{Perlin, Seedable};

pub fn generate(_rb_module: Value) -> Value {
    let perlin = Perlin::new();
    perlin.set_seed(1564863213);

    let mut mesh = PolyMesh::new(Some(128), Some(128), Some(10), Some(10));
    let size = 128;
    let noise_map = generate_noise_map(perlin, size, size, 128.0, 5);
    mesh.displace_with_noise_map(noise_map, size, size);
    mesh.calculate_normals();

    //let result = rb_ary_new_capa(mesh.vertices)

    return unsafe { rb_int2inum(200) };
}
