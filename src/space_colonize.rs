use na::{Pnt3, FloatPnt, Vec3, Norm, normalize};
use std::cmp::Ordering::{Less, Equal, Greater};
use std::cmp::Ordering;
use std::f32;
use std::num::Float;
use std::collections::HashMap;
use std::iter::AdditiveIterator;
use std::iter::Peekable;
use std::rand;

#[derive(Debug)]
pub struct Node {
    pos: Pnt3<f32>,
    children: Vec<Node>,
    hops: u32
}

fn closest_node(nodes: &Vec<Pnt3<f32>>, pnt: Pnt3<f32>) -> (Pnt3<f32>, f32) {
    nodes.iter()
        .map(|&base| (base, base.dist(&pnt)))
        .fold( (Pnt3::new(0.0f32, 0.0, 0.0), f32::INFINITY), |(pnt, accum), (base, dist)|
              if dist < accum { (base, dist) } else { (pnt, accum) }
             )
}

#[derive(Debug)]
struct PntMap<T> {
    pnts : Vec<(Pnt3<f32>, Vec<T>)>
}

impl<T> PntMap<T> where T : Clone{
    fn add_point(&mut self, pnt: Pnt3<f32>, val : T) {
        let search : Vec<(usize, (Pnt3<f32>, Vec<T>))>= self.pnts.clone().into_iter()
            .enumerate()
            .filter(|&(i, (base, _))| base.dist(&pnt) <= 1e-10)
            .collect();

        match search.as_slice() {
            [] => self.pnts.push((pnt, vec!(val))),
            [(i, _)] => self.pnts[i].1.push(val),
            x => panic!("Should never have duplicate values in map")
        };
    }

    fn new() -> PntMap<T> {
        PntMap{pnts: vec!()}
    }
}


pub fn space_colonize(source: &Vec<Pnt3<f32>>, dest: &Vec<Pnt3<f32>>, step: f32, radius: f32) -> Vec<Pnt3<f32>> {
    //let mut nodes = source.iter().map(|pnt| {
        //Node{pos: pnt.clone(), children: vec!(), hops: 0}
    //}).collect();

    let mut flat_nodes = source.clone();

    let mut targets : Vec<Pnt3<f32>> = dest.clone();

    let mut iter = 0;
    while targets.len() > 0 && iter <= 100 {
        // Grab the closest source node to each target
        let closest_source_target : Vec<(Pnt3<f32>, Pnt3<f32>)> = targets.iter()
            .map(|&target| {
                let (source, dist) = closest_node(&flat_nodes, target);
                (source, target)
            })
            .collect();
        // average the direction out of the source nodes that have duplicate target nodes


        let mut pnt_map = PntMap::new();
        for (source, target) in closest_source_target {
            pnt_map.add_point(source, target);
        }

        for (source, targets) in pnt_map.pnts {
            let normed_dirs = targets.iter().map(|pnt| normalize(&(*pnt - source)));
            let summed_dir = normed_dirs.fold(Vec3::new(0., 0., 0.), |pnt, accum| accum + pnt);
            let source_target_dir = normalize(&summed_dir) + rand::random() * 0.01;

            // random noise is to break symetry
            flat_nodes.push(source + source_target_dir * step + rand::random() * 0.01);
            println!("Last {:?}", flat_nodes.last())
        }


        targets = targets.into_iter().filter(|&pnt| closest_node(&flat_nodes, pnt).1 >= radius).collect();
        println!("{:?}", targets);

        iter += 1;
    }

    // add a little random noise to break symetry

    flat_nodes
}


#[cfg(test)]
mod test {
    use na::{Pnt3, FloatPnt, Zero};
    use super::{closest_node, space_colonize};
    use super::PntMap;

    #[test]
    fn test_closet_nodes() {
        let nodes = vec!(Pnt3::new(1.0, 2.0, 3.0), Pnt3::new(0.1, 0.1, 0.1), Pnt3::new(1.0, 1.0, 1.0));
        let point = Pnt3::new(0.0, 0.0, 0.0);
        let (closest, dist) = closest_node(&nodes, point);
        assert_eq!(closest, Pnt3::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_PntMap() {
        let mut pnt_map = PntMap::new();
        pnt_map.add_point(Pnt3::new(0.0, 0.0, 0.0), 1);
        pnt_map.add_point(Pnt3::new(0.0, 0.0, 0.0), 2);
        pnt_map.add_point(Pnt3::new(1.0, 0.0, 0.0), 2);

        assert_eq!(pnt_map.pnts[0].1, vec!(1, 2));
        assert_eq!(pnt_map.pnts[1].1, vec!(2));
    }

    #[test]
    fn test_space_colonize() {
        let source = vec!(Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0));
        let dest = vec!(Pnt3::new(3.0, 0.0, 1.0), Pnt3::new(-0.1, 0.0, 1.0), Pnt3::new(-0.1, 0.0, 1.3));
        let step = 0.3;
        let radius = 0.4;
        let res = space_colonize(&source, &dest, step, radius);
        println!("{:?}", res);
        assert_eq!(0, 1);
    }
}
