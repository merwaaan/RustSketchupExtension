use crate::ruby::{RubyArray, RubyFloat, RubyInt, Value, NIL};
use rapier3d::prelude::*;

struct Object {
    body: RigidBody,
    collider: Collider,
}

static mut STATIC_OBJECTS: Vec<Object> = vec![];
static mut DYNAMIC_OBJECTS: Vec<Object> = vec![];

fn create_objects(rb_objects: Value, is_static: bool) {
    unsafe {
        // TODO?
        if is_static {
            STATIC_OBJECTS.clear();
        } else {
            DYNAMIC_OBJECTS.clear();
        }
    }

    let rb_objects: RubyArray = rb_objects.into();

    for object_index in 0..rb_objects.length() {
        let rb_object: RubyArray = rb_objects.at(object_index).into();

        // ID

        let rb_id = rb_object.at(0);
        let id: i64 = rb_id.into();

        // Center

        let rb_center: RubyArray = rb_object.at(1).into();

        let x: f32 = rb_center.at(0).into();
        let y: f32 = rb_center.at(1).into();
        let z: f32 = rb_center.at(2).into();

        let center = vector![x as f32, y as f32, z as f32];

        let builder = if is_static {
            RigidBodyBuilder::fixed()
        } else {
            RigidBodyBuilder::dynamic()
        };

        let body = builder.user_data(id as u128).translation(center).build();

        // Size

        let rb_size: RubyArray = rb_object.at(2).into();

        let sx: f32 = rb_size.at(0).into();
        let sy: f32 = rb_size.at(1).into();
        let sz: f32 = rb_size.at(2).into();

        let collider = ColliderBuilder::cuboid(sx * 0.5, sy * 0.5, sz * 0.5).build();

        let object = Object { body, collider };

        unsafe {
            // TODO?
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

    return NIL;
}

pub fn set_dynamic_objects(_rb_module: Value, rb_objects: Value) -> Value {
    create_objects(rb_objects, false);

    return NIL;
}

pub fn simulate(_rb_module: Value, rb_frame_count: Value) -> Value {
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

    let object_count = unsafe { STATIC_OBJECTS.len() + DYNAMIC_OBJECTS.len() };

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

        let rb_frame = RubyArray::with_capacity(object_count);

        for (_handle, body) in bodies.iter() {
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
