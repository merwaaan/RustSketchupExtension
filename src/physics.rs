use rapier3d::prelude::*;

use crate::ruby::{
    rb_ary_entry, rb_ary_len, rb_ary_new_capa, rb_ary_push, rb_float_new, rb_num2dbl, Value,
};

pub fn simulate(_rb_module: Value, rb_scene: Value) -> Value {
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
        for i in 0..rb_ary_len(rb_scene) {
            let rb_position = rb_ary_entry(rb_scene, i);

            let position = vector![
                rb_num2dbl(rb_ary_entry(rb_position, 0)) as f32,
                rb_num2dbl(rb_ary_entry(rb_position, 1)) as f32,
                rb_num2dbl(rb_ary_entry(rb_position, 2)) as f32
            ];

            let body = RigidBodyBuilder::dynamic().translation(position).build();
            let body_handle = bodies.insert(body);

            let collider = ColliderBuilder::ball(1.0).build();

            colliders.insert_with_parent(collider, body_handle, &mut bodies);
        }
    }

    let rb_frames = unsafe { rb_ary_new_capa(100) };

    for _ in 0..100 {
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
            let rb_frame = rb_ary_new_capa(1);

            for (_handle, body) in bodies.iter() {
                let rb_position = rb_ary_new_capa(3);
                rb_ary_push(rb_position, rb_float_new(body.translation().x.into()));
                rb_ary_push(rb_position, rb_float_new(body.translation().y.into()));
                rb_ary_push(rb_position, rb_float_new(body.translation().z.into()));
                rb_ary_push(rb_frame, rb_position);
            }

            rb_ary_push(rb_frames, rb_frame);
        }
    }

    return rb_frames;
}
