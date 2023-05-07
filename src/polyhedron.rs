use polyhedron_ops::Polyhedron;

use crate::ruby::{array::RubyArray, numeric::RubyFloat, Value};

pub fn generate_polyhedron(_rb_module: Value) -> Value {
    let polyhedron = Polyhedron::dodecahedron()
        .chamfer(None, true)
        .propellor(None, true)
        .ambo(None, true)
        .gyro(None, None, true)
        .triangulate(Some(true))
        .finalize();

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
