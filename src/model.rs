extern crate stl;

use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use na::{Vec3, Vec4, Rot3, Pnt3, Mat4, Eye, Zero, Mat, ToHomogeneous};
use na::cross;
use std::old_io::{File, IoResult};


pub struct Tri {
    pub normal : Vec3<f32>,
    pub v1 : Pnt3<f32>,
    pub v2 : Pnt3<f32>,
    pub v3 : Pnt3<f32>
}

impl Tri {
    pub fn new(v1: &Pnt3<f32>, v2: &Pnt3<f32>, v3: &Pnt3<f32>) -> Tri {
        let normal = cross(v1.as_vec(), v2.as_vec());
        Tri {normal: normal, v1: v1.clone(), v2: v2.clone(), v3: v3.clone()}
    }

    pub fn to_stl_triangle(&self) -> stl::Triangle {
        stl::Triangle {
            normal: self.normal.as_array().clone(),
            v1: self.v1.as_array().clone(),
            v2: self.v2.as_array().clone(),
            v3: self.v3.as_array().clone(),
            attr_byte_count: 0
        }
    }

    pub fn transform(&self, matrix : Mat4<f32>) -> Tri {
        fn trans(p : Pnt3<f32>, matrix : Mat4<f32>) -> Pnt3<f32> {
            let v = Vec4::new(p.x, p.y, p.z, 1.0);
            let v = matrix * v;
            Pnt3::new(v.x, v.y, v.z)
        }

        let v1 = trans(self.v1, matrix);
        let v2 = trans(self.v2, matrix);
        let v3 = trans(self.v3, matrix);

        Tri::new(&v1, &v2, &v3)
    }
}

pub struct Mesh {
    pub tris : Vec<Tri>
}

impl Mesh {
    pub fn to_stl_triangles(&self) -> Vec<stl::Triangle> {
        self.tris.iter().map(Tri::to_stl_triangle).collect()
    }
}

#[derive(Debug)]
pub struct Cuboid {
    pub size : Vec3<f32>,
    pub trans : Mat4<f32>
}

impl Cuboid {
    pub fn new(size : Vec3<f32>, translation : Mat4<f32>) -> Cuboid {
        Cuboid{size: size, trans: translation}
    }

    pub fn to_mesh(&self) -> Mesh {
        let p1 = Pnt3::new(-1., -1., -1.);
        let p2 = Pnt3::new(-1., 1., -1.);
        let p3 = Pnt3::new(1., 1., -1.);
        let p4 = Pnt3::new(1., -1., -1.);
        let p5 = Pnt3::new(1., -1., 1.);
        let p6 = Pnt3::new(-1., -1., 1.);
        let p7 = Pnt3::new(-1., 1., 1.);
        let p8 = Pnt3::new(1., 1., 1.);

        let tris = vec!(
            Tri::new(&p2, &p3, &p1),
            Tri::new(&p3, &p4, &p1),
            Tri::new(&p5, &p6, &p1),
            Tri::new(&p1, &p4, &p5),
            Tri::new(&p8, &p7, &p5),
            Tri::new(&p7, &p6, &p5),
            Tri::new(&p2, &p7, &p3),
            Tri::new(&p7, &p8, &p3),
            Tri::new(&p3, &p8, &p5),
            Tri::new(&p3, &p5, &p4),
            Tri::new(&p6, &p7, &p2),
            Tri::new(&p2, &p1, &p6)
            );

        let tris = tris.iter().map(|tri| tri.transform(self.trans)).collect();

        Mesh {tris: tris}
    }
}

#[derive(Debug)]
pub struct World {
    pub nodes : Vec<Cuboid>
}

impl World {
    pub fn new() -> World {
        World {nodes : vec!()}
    }

    pub fn add_cube(&mut self, obj : Cuboid) {
        self.nodes.push(obj);
    }

    pub fn write_to_stl(&self, path : &Path) -> IoResult<()> {
        let mut tris : Vec<stl::Triangle> = vec!();
        let meshes = self.nodes.iter().map(Cuboid::to_mesh);
        for mesh in meshes {
            let iter = mesh.to_stl_triangles().into_iter();
            tris.extend(iter);
        }

        let stl_file = stl::BinaryStlFile {
            header: stl::BinaryStlHeader { header: [0u8; 80],
                                           num_triangles: tris.len() as u32},
            triangles: tris
        };
        let mut file = File::create(path);
        stl::write_stl(&mut file, &stl_file)
    }
}
pub trait Transform {
    fn eye() -> Self;
    fn translate(&self, translate: Vec3<f32>) -> Self;
    fn scale(&self, scale: Vec3<f32>) -> Self;
    fn rotate(&self, axisAngle: Vec3<f32>) -> Self;
}

fn test() {
    let trans : Mat4<f32> = Zero::zero();
    let mut t2 : Mat4<f32> = Zero::zero();
    println!("{:?}", t2 + trans);
}

impl Transform for Mat4<f32> {
    fn eye() -> Mat4<f32> {
        Eye::new_identity(4)
    }

    fn translate(&self, translate: Vec3<f32>) -> Mat4<f32> {
        let mut trans : Mat4<f32> = Zero::zero();
        trans[(0, 3)] = translate.x;
        trans[(1, 3)] = translate.y;
        trans[(2, 3)] = translate.z;

        trans + (*self)
    }

    fn scale(&self, scale: Vec3<f32>) -> Mat4<f32> {
        let mut s = self.clone();
        s[(0, 0)] *= scale.x;
        s[(1, 0)] *= scale.x;
        s[(2, 0)] *= scale.x;

        s[(0, 1)] *= scale.y;
        s[(1, 1)] *= scale.y;
        s[(2, 1)] *= scale.y;

        s[(0, 2)] *= scale.z;
        s[(1, 2)] *= scale.z;
        s[(2, 2)] *= scale.z;
        s
    }

    fn rotate(&self, axisangle: Vec3<f32>) -> Mat4<f32> {
        let mat = Rot3::new(axisangle).to_homogeneous();
        (*self) * mat
    }
}

#[cfg(test)]
mod test {
    use super::{Cuboid, World};

    #[test]
    fn add_cube_to_world() {
    }
}
