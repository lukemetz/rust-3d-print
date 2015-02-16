extern crate "rust-model" as model;
extern crate "nalgebra" as na;

use model::{Cuboid, World, Transform, space_colonize};
use na::{Vec3, Pnt3, Eye, Mat4, FloatPnt};
use std::rand;

fn main() {
    let mut world = World::new();

    let mut dest_pnts = vec!();
    for k in range(0, 2000) {
        //dest_pnts.push(Pnt3::new(k as f32, k as f32, 0.0));
        let pnt : Pnt3<f32> = (std::rand::random::<Pnt3<f32>>() - 0.5) * 10.0;
        let circle = pnt.dist(&Pnt3::new(0.0, 0.0, 0.0));
        if circle >= 4.5 && circle < 5.0 {
            dest_pnts.push(pnt)
        }
    }

    let mut src_pnts = vec!(Pnt3::new(0.0, 0.0, 0.0));
    //for k in range(0, 4) {
        //src_pnts.push(Pnt3::new(k as f32, -1.0, 0.0));
    //}

    let render_pnts = space_colonize(&src_pnts, &dest_pnts, 0.1, 0.2);
    for pnt in render_pnts {
        let t : Mat4<f32> = Transform::eye();
        let t = t.scale(Vec3::new(0.1, 0.1, 0.1))
            .translate(pnt.to_vec());
        let cube = Cuboid::new(Vec3::new(0.5, 0.5, 0.5), t);
        world.add_cube(cube);
    }


    println!("Hello, world!");
    println!("{:?}", world);
    world.write_to_stl(&Path::new("test.stl"));
}
