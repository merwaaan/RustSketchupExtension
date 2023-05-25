use crate::ruby::{RubyArray, RubyFloat, Value};
use perlin2d::PerlinNoise2D;
use rand::prelude::*;

pub fn generate(_rb_module: Value, rb_resolution: Value) -> Value {
    let resolution: usize = rb_resolution.into();

    let mut random = rand::thread_rng();

    let perlin = PerlinNoise2D::new(
        6,
        10.0,
        0.5,
        1.0,
        2.0,
        (resolution as f64, resolution as f64),
        0.5,
        random.gen_range(1..1000),
    );

    let triangle_count = resolution.saturating_sub(1).pow(2) * 2;

    let rb_mesh = RubyArray::with_capacity(triangle_count);

    for y in 0..resolution.saturating_sub(1) {
        for x in 0..resolution.saturating_sub(1) {
            let triangle1_offsets = [0usize, 1usize, 2usize];
            let triangle2_offsets = [1usize, 3usize, 2usize];

            for triangle_offsets in [triangle1_offsets, triangle2_offsets] {
                let rb_triangle = RubyArray::with_capacity(3);

                for vertex_offset in triangle_offsets {
                    let x = (x + vertex_offset % 2) as f64;
                    let y = (y + vertex_offset / 2) as f64;
                    let z = perlin.get_noise(x, y);

                    let rb_vertex = RubyArray::with_capacity(3);
                    rb_vertex.push(RubyFloat::new(x));
                    rb_vertex.push(RubyFloat::new(y));
                    rb_vertex.push(RubyFloat::new(z));
                    rb_triangle.push(rb_vertex);
                }

                rb_mesh.push(rb_triangle);
            }
        }
    }

    rb_mesh.into()
}
