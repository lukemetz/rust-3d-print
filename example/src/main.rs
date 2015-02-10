extern crate "rust-model" as model;
extern crate "nalgebra" as na;

use model::{Cuboid, World};
use na::{Vec3, Pnt3, Eye, Mat4};

fn main() {
    let cube = Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Eye::new_identity(4));
    let mut mat : Mat4<f32> = Eye::new_identity(4);
    mat[(0, 3)] = 3.0;
    mat[(3, 0)] = 3.0;
    let cube2 = Cuboid::new(Vec3::new(0.0, 0.0, 0.0), mat);
    let mut world = World::new();
    println!("{:?}", cube);
    world.add_cube(cube);
    world.add_cube(cube2);

    println!("Hello, world!");
    println!("{:?}", world);
    world.write_to_stl(&Path::new("test.stl"));
}
