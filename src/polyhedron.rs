use polyhedron_ops::Polyhedron;
use rand::Rng;

use crate::ruby::{array::RubyArray, numeric::RubyFloat, Value};

pub fn generate_polyhedron(_rb_module: Value) -> Value {
    let mut random = rand::thread_rng();

    let mut polyhedron = match random.gen_range(0..5) {
        0 => Polyhedron::tetrahedron(),
        1 => Polyhedron::hexahedron(),
        2 => Polyhedron::octahedron(),
        3 => Polyhedron::dodecahedron(),
        _ => Polyhedron::icosahedron(),
    };

    for _ in 0..3 {
        match random.gen_range(0..4) {
            0 => polyhedron.ambo(None, true),
            1 => polyhedron.propellor(None, true),
            2 => polyhedron.ambo(None, true),
            _ => polyhedron.gyro(None, None, true),
        };
    }

    polyhedron.triangulate(None).finalize();

    let rb_faces = RubyArray::with_capacity(polyhedron.faces().len());

    for face in polyhedron.faces() {
        let rb_face = RubyArray::with_capacity(face.len());

        for vertex_index in face {
            let vertex = polyhedron.positions()[*vertex_index as usize];

            let rb_vertex = RubyArray::with_capacity(3);
            rb_vertex.push(RubyFloat::new(vertex.x as f64));
            rb_vertex.push(RubyFloat::new(vertex.y as f64));
            rb_vertex.push(RubyFloat::new(vertex.z as f64));

            rb_face.push(rb_vertex);
        }

        rb_faces.push(rb_face)
    }

    return rb_faces.into();
}
