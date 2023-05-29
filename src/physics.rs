use crate::ruby::{RubyArray, RubyFloat, RubyInt, Value, NIL};
use rapier3d::prelude::*;
use std::cell::RefCell;

struct Object {
    body: RigidBody,
    collider: Collider,
}

thread_local!(
    static STATIC_OBJECTS: RefCell<Vec<Object>> = RefCell::new(vec![]);
    static DYNAMIC_OBJECTS: RefCell<Vec<Object>> = RefCell::new(vec![]);
);

fn create_objects(rb_objects: Value, is_static: bool) {
    let objects = if is_static {
        &STATIC_OBJECTS
    } else {
        &DYNAMIC_OBJECTS
    };

    objects.with(|objects| {
        objects.borrow_mut().clear();
    });

    let rb_objects: RubyArray = rb_objects.into();

    for object_index in 0..rb_objects.length() {
        let rb_object: RubyArray = rb_objects.at(object_index).into();

        // ID

        let rb_id = rb_object.at(0);
        let id: i64 = rb_id.into();

        // Transformation

        let rb_position: RubyArray = rb_object.at(1).into();

        let x: f64 = rb_position.at(0).into();
        let y: f64 = rb_position.at(1).into();
        let z: f64 = rb_position.at(2).into();

        let position = vector![x as f32, y as f32, z as f32];

        let rb_rotation: RubyArray = rb_object.at(2).into();

        let rx: f64 = rb_rotation.at(0).into();
        let ry: f64 = rb_rotation.at(1).into();
        let rz: f64 = rb_rotation.at(2).into();

        let rotation = vector![rx as f32, ry as f32, rz as f32];

        let builder = if is_static {
            RigidBodyBuilder::fixed()
        } else {
            RigidBodyBuilder::dynamic()
        };

        let body = builder
            .user_data(id as u128)
            .translation(position)
            .rotation(rotation)
            .ccd_enabled(true)
            .build();

        // Geometry

        let rb_triangles: RubyArray = rb_object.at(3).into();

        let mut vertices: Vec<nalgebra::OPoint<f32, nalgebra::Const<3>>> = Vec::new();
        let mut indices: Vec<[u32; 3]> = Vec::new();

        for triangle_index in 0..rb_triangles.length() {
            let rb_triangle: RubyArray = rb_triangles.at(triangle_index).into();

            for vertex_index in 0..3 {
                let rb_vertex: RubyArray = rb_triangle.at(vertex_index).into();

                let x: f64 = rb_vertex.at(0).into();
                let y: f64 = rb_vertex.at(1).into();
                let z: f64 = rb_vertex.at(2).into();

                let vertex = point![x as f32, y as f32, z as f32];

                vertices.push(vertex);
            }

            indices.push([
                triangle_index as u32 * 3,
                triangle_index as u32 * 3 + 1,
                triangle_index as u32 * 3 + 2,
            ]);
        }

        let collider = ColliderBuilder::trimesh(vertices, indices)
            .restitution(0.5)
            .build();

        let object = Object { body, collider };

        objects.with(|objects| {
            objects.borrow_mut().push(object);
        });
    }
}

pub fn set_static_objects(_rb_module: Value, rb_objects: Value) -> Value {
    create_objects(rb_objects, true);

    return NIL;
}

pub fn set_dynamic_objects(_rb_module: Value, rb_objects: Value) -> Value {
    create_objects(rb_objects, false);

    return NIL;
}

pub fn simulate(_rb_module: Value, rb_frame_count: Value) -> Value {
    // Setup the simulation

    let gravity = vector![0.0, 0.0, -9.81 * 39.3701];

    let mut integration_params = IntegrationParameters::default();
    integration_params.dt = 1.0 / 60.0;

    let mut pipeline = PhysicsPipeline::new();
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

    // Insert objects

    let object_count = DYNAMIC_OBJECTS.with(|objects| objects.borrow().len());

    for objects in &[STATIC_OBJECTS, DYNAMIC_OBJECTS] {
        objects.with(|objects| {
            for object in objects.borrow().iter() {
                let body_handle = bodies.insert(object.body.clone());
                colliders.insert_with_parent(object.collider.clone(), body_handle, &mut bodies);
            }
        });
    }

    // Run

    let frame_count: i64 = rb_frame_count.into();

    let rb_frames = RubyArray::with_capacity(frame_count as usize);

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

        // Convert the frame to Ruby

        let rb_frame = RubyArray::with_capacity(object_count);

        for (_handle, body) in bodies.iter() {
            if !body.is_dynamic() {
                continue;
            }

            let rb_object_data = RubyArray::with_capacity(3);

            // ID

            rb_object_data.push(RubyInt::new(body.user_data as i64));

            // Position

            let position = body.translation().clone();

            let rb_position = RubyArray::with_capacity(3);
            rb_position.push(RubyFloat::new(position.x.into()));
            rb_position.push(RubyFloat::new(position.y.into()));
            rb_position.push(RubyFloat::new(position.z.into()));
            rb_object_data.push(rb_position);

            // Rotation

            let maybe_axis_angle = body.rotation().axis_angle();

            let rb_rotation = RubyArray::with_capacity(4);

            if let Some(axis_angle) = maybe_axis_angle {
                rb_rotation.push(RubyFloat::new(axis_angle.0[0].into()));
                rb_rotation.push(RubyFloat::new(axis_angle.0[1].into()));
                rb_rotation.push(RubyFloat::new(axis_angle.0[2].into()));
                rb_rotation.push(RubyFloat::new(axis_angle.1.into()));
            } else {
                rb_rotation.push(RubyFloat::new(1.0));
                rb_rotation.push(RubyFloat::new(0.0));
                rb_rotation.push(RubyFloat::new(0.0));
                rb_rotation.push(RubyFloat::new(0.0));
            }

            rb_object_data.push(rb_rotation);

            rb_frame.push(rb_object_data);
        }

        rb_frames.push(rb_frame);
    }

    return rb_frames.into();
}
