use polyhedron_ops::Polyhedron;

use crate::ruby::{rb_ary_new_capa, rb_ary_push, rb_float_new, rb_int2inum, Value};

pub fn generate_polyhedron(_rb_module: Value) -> Value {
    let polyhedron = Polyhedron::dodecahedron()
        .chamfer(None, true)
        .propellor(None, true)
        .ambo(None, true)
        .gyro(None, None, true)
        .triangulate(Some(true))
        .finalize();

    let rb_faces = unsafe { rb_ary_new_capa(polyhedron.faces().len() as libc::c_long) };

    for face in polyhedron.faces() {
        let rb_face = unsafe { rb_ary_new_capa(face.len() as libc::c_long) };

        for vertex_index in face {
            let vertex = polyhedron.positions()[*vertex_index as usize];

            unsafe {
                let rb_vertex = rb_ary_new_capa(3);
                rb_ary_push(rb_vertex, rb_float_new(vertex.x.into()));
                rb_ary_push(rb_vertex, rb_float_new(vertex.y.into()));
                rb_ary_push(rb_vertex, rb_float_new(vertex.z.into()));

                rb_ary_push(rb_face, rb_vertex);
            }
        }

        unsafe { rb_ary_push(rb_faces, rb_face) };
    }

    return rb_faces;
}
