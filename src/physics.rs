use rapier3d::prelude::*;

use crate::ruby::{
    rb_ary_entry, rb_ary_len, rb_ary_new_capa, rb_ary_push, rb_float_new, rb_int2inum, rb_num2dbl,
    rb_num2int, Value,
};

struct Object {
    id: i32,
    body: RigidBody,
    collider: Collider,
}

static mut STATIC_OBJECTS: Vec<Object> = vec![];
static mut DYNAMIC_OBJECTS: Vec<Object> = vec![];

fn create_objects(rb_objects: Value, is_static: bool) {
    unsafe {
        for i in 0..rb_ary_len(rb_objects) {
            let rb_object = rb_ary_entry(rb_objects, i);

            // ID

            let rb_id = rb_ary_entry(rb_object, 0);
            let id = rb_num2int(rb_id);

            // Transformation

            let rb_position = rb_ary_entry(rb_object, 1);
            let position = vector![
                rb_num2dbl(rb_ary_entry(rb_position, 0)) as f32,
                rb_num2dbl(rb_ary_entry(rb_position, 1)) as f32,
                rb_num2dbl(rb_ary_entry(rb_position, 2)) as f32
            ];

            let builder = if is_static {
                RigidBodyBuilder::fixed()
            } else {
                RigidBodyBuilder::dynamic()
            };

            let body = builder.user_data(id as u128).translation(position).build();

            // Geometry

            let rb_triangles = rb_ary_entry(rb_object, 2);

            let mut vertices: Vec<nalgebra::OPoint<f32, nalgebra::Const<3>>> = Vec::new();
            let mut indices: Vec<[u32; 3]> = Vec::new();

            for triangle_index in 0..rb_ary_len(rb_triangles) {
                let rb_triangle = rb_ary_entry(rb_triangles, triangle_index);

                for vertex_index in 0..3 {
                    let rb_vertex = rb_ary_entry(rb_triangle, vertex_index);

                    let vertex = point![
                        rb_num2dbl(rb_ary_entry(rb_vertex, 0)) as f32,
                        rb_num2dbl(rb_ary_entry(rb_vertex, 1)) as f32,
                        rb_num2dbl(rb_ary_entry(rb_vertex, 2)) as f32
                    ];

                    vertices.push(vertex);
                }

                indices.push([
                    triangle_index as u32 * 3,
                    triangle_index as u32 * 3 + 1,
                    triangle_index as u32 * 3 + 2,
                ]);
            }

            let collider = ColliderBuilder::trimesh(vertices, indices).build();

            let object = Object { id, body, collider };

            if is_static {
                STATIC_OBJECTS.push(object);
            } else {
                DYNAMIC_OBJECTS.push(object);
            }
        }
    }
}

pub fn set_static_objects(_rb_module: Value, rb_objects: Value) -> Value {
    create_objects(rb_objects, true);

    return unsafe { rb_ary_new_capa(0) };
}

pub fn set_dynamic_objects(_rb_module: Value, rb_objects: Value) -> Value {
    create_objects(rb_objects, false);

    return unsafe { rb_ary_new_capa(0) };
}

pub fn simulate(_rb_module: Value, rb_frame_count: Value) -> Value {
    let mut pipeline = PhysicsPipeline::new();
    let gravity = vector![0.0, 0.0, -9.81];
    let integration_params = IntegrationParameters::default();
    let mut island_manager = IslandManager::new();
    let mut broadphase = BroadPhase::new();
    let mut narrowphase = NarrowPhase::new();
    let mut impulse_joints = ImpulseJointSet::new();
    let mut multibody_joints = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();

    unsafe {
        for object in &STATIC_OBJECTS {
            let body_handle = bodies.insert(object.body.clone());

            colliders.insert_with_parent(object.collider.clone(), body_handle, &mut bodies);
        }

        for object in &DYNAMIC_OBJECTS {
            let body_handle = bodies.insert(object.body.clone());

            colliders.insert_with_parent(object.collider.clone(), body_handle, &mut bodies);
        }
    }

    let frame_count = unsafe { rb_num2int(rb_frame_count) };

    let rb_frames = unsafe { rb_ary_new_capa(frame_count) };

    for _ in 0..frame_count {
        pipeline.step(
            &gravity,
            &integration_params,
            &mut island_manager,
            &mut broadphase,
            &mut narrowphase,
            &mut bodies,
            &mut colliders,
            &mut impulse_joints,
            &mut multibody_joints,
            &mut ccd_solver,
            None,
            &physics_hooks,
            &event_handler,
        );

        unsafe {
            let rb_frame: Value = rb_ary_new_capa(1); // TODO obj count

            for (_handle, body) in bodies.iter() {
                let rb_object_data = rb_ary_new_capa(2);

                // ID

                rb_ary_push(rb_object_data, rb_int2inum(body.user_data as isize));

                // Transformation

                let rb_position = rb_ary_new_capa(3);
                rb_ary_push(rb_position, rb_float_new(body.translation().x.into()));
                rb_ary_push(rb_position, rb_float_new(body.translation().y.into()));
                rb_ary_push(rb_position, rb_float_new(body.translation().z.into()));
                rb_ary_push(rb_object_data, rb_position);

                rb_ary_push(rb_frame, rb_object_data);
            }

            rb_ary_push(rb_frames, rb_frame);
        }
    }

    return rb_frames;
}
