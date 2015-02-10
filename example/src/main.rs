extern crate "rust-model" as model;
extern crate "nalgebra" as na;

use model::{Cuboid, World};
use na::{Vec3, Pnt3, Eye};

fn main() {
    let cube = Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Eye::new_identity(4));
    let mut world = World::new();
    println!("{:?}", cube);
    world.add_cube(cube);

    println!("Hello, world!");
    println!("{:?}", world);
}
