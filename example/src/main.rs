extern crate "rust-model" as model;
extern crate "nalgebra" as na;

use model::{Cuboid, World, Transform};
use na::{Vec3, Pnt3, Eye, Mat4};

fn main() {
    let cube = Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Eye::new_identity(4));
    let mut mat : Mat4<f32> = Eye::new_identity(4);
    mat[(0, 3)] = 3.0;
    mat[(3, 0)] = 3.0;
    let cube2 = Cuboid::new(Vec3::new(0.0, 0.0, 0.0), mat);
    let mut world = World::new();
    world.add_cube(cube);
    world.add_cube(cube2);

    for x in range(0, 30) {
        let t : Mat4<f32> = Transform::eye();
        let t = t.rotate(Vec3::new(1.0, 0.0, 0.0) * 1.3 * (x as f32))
            .translate(Vec3::new(1.1 * (x as f32), 0.0, 0.0))
            .scale(Vec3::new(1.0, 1.0, 1.0));
        let cube = Cuboid::new(Vec3::new(0.0, 0.0, 0.0), t);
        world.add_cube(cube);
    }

    println!("Hello, world!");
    println!("{:?}", world);
    world.write_to_stl(&Path::new("test.stl"));
}
