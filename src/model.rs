use na::{Vec3, Rot3, Pnt3, Mat4};

#[derive(Debug)]
pub struct Cuboid {
    pub size : Vec3<f64>,
    pub trans : Mat4<f64>
}

pub struct Tri {
    pub normal : Vec3<f64>,
    pub v1 : Pnt3<f64>,
    pub v2 : Pnt3<f64>,
    pub v3 : Pnt3<f64>
}

impl Tri {
    pub fn new(v1: &Pnt3<f64>, v2: &Pnt3<f64>, v3: &Pnt3<f64>) {
        v1.as_vec()
    }
}

impl Cuboid {
    pub fn new(size : Vec3<f64>, translation : Mat4<f64>) -> Cuboid {
        Cuboid{size: size, trans: translation}
    }

    pub fn to_triangle(&self) -> Vec<stl::Triangle> {
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

    pub write_to_stl(&self) {

    }
}

#[cfg(test)]
mod test {
    use super::{Cuboid, World};

    #[test]
    fn add_cube_to_world() {
    }
}
