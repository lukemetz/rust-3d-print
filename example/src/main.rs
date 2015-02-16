extern crate "rust-model" as model;
extern crate "nalgebra" as na;
extern crate image;

use model::{Cuboid, World, Transform, space_colonize};
use na::{Vec3, Pnt3, Eye, Mat4, FloatPnt};
use std::rand;

static scale : f32 = 100.0;

fn main() {
    let mut world = World::new();


    fn make_dest(path : &Path) -> Vec<Pnt3<f32>> {
        let image = image::open(path).unwrap();
        let image = image.to_rgba();
        let mut dest_pnts = vec!();
        for k in range(0, 2000) {
            let x = (std::rand::random::<f32>() * (image.width() as f32));
            let sx = x as u32;
            let y = (std::rand::random::<f32>() * (image.height() as f32));
            let sy = y as u32;

            let x = x / scale;
            let y = y / scale;

            let color = image.get_pixel(sx, sy);
            if color[0] != 255 || color[1] != 255 || color[2] != 255 {
                let pnt = Pnt3::new(x, y, std::rand::random::<f32>() * 1.0);
                dest_pnts.push(pnt);
            }
        }
        dest_pnts
    }

    let image = image::open(&Path::new("olin.png")).unwrap().to_rgba();
    let start_y = (image.height() as f32) / 2.0 / scale;
    let max_x = (image.width() as f32) / scale;

    let mut src_pnts_l= vec!(Pnt3::new(0.0, start_y, 0.0));
    let mut src_pnts_r = vec!(Pnt3::new(max_x, start_y, 0.0));

    let dest_pnts_l = make_dest(&Path::new("olin_l.png"));
    let render_l = space_colonize(&src_pnts_l, &dest_pnts_l, 0.05, 0.051);

    let dest_pnts_r = make_dest(&Path::new("olin_r.png"));
    let render_r = space_colonize(&src_pnts_r, &dest_pnts_r, 0.05, 0.051);

    let mut render_pnts = vec!();

    render_pnts.push_all(&render_l);
    render_pnts.push_all(&render_r);

    for pnt in render_pnts {
        let t : Mat4<f32> = Transform::eye();
        let t = t.scale(Vec3::new(0.05, 0.05, 0.05))
            .translate(pnt.to_vec());
        let cube = Cuboid::new(Vec3::new(0.5, 0.5, 0.5), t);
        world.add_cube(cube);
    }


    //println!("Hello, world!");
    //println!("{:?}", world);
    world.write_to_stl(&Path::new("test.stl"));
}
