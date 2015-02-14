use na::{Pnt3, FloatPnt, Vec3, Norm, normalize};
use std::cmp::Ordering::{Less, Equal, Greater};
use std::cmp::Ordering;
use std::f32;
use std::num::Float;
use std::collections::HashMap;
use std::iter::AdditiveIterator;
use std::iter::Peekable;

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


pub fn space_colonize(source: &Vec<Pnt3<f32>>, dest: &Vec<Pnt3<f32>>, step: f32, radius: f32) -> Vec<Node> {
    let mut nodes = source.iter().map(|pnt| {
        Node{pos: pnt.clone(), children: vec!(), hops: 0}
    }).collect();

    let flat_nodes = source.clone();

    let mut targets : Vec<Pnt3<f32>> = dest.clone();

    while targets.len() > 0 {
        // Grab the closest nodes to each target
        let closest : Vec<(Pnt3<f32>, f32)> = flat_nodes.iter()
            .map(|&node| {
                closest_node(&targets, node)
            })
            .collect();
        //average the closest nodes amound targets

        let mut new_to_add : Vec<(Pnt3<f32>, Vec3<f32>)> = vec!();
        for target in targets.clone() {
            let closest_pnts_iter = closest.iter()
                .map(|&(pnt, dist)| pnt)
                .filter(|&pnt| pnt.dist(&target) <= 1e-5);
            let closest_pnts : Vec<Pnt3<f32>> = closest_pnts_iter.collect();

            if closest_pnts.len() <= 0 {
                let normed_dirs = closest_pnts.iter().map(|pnt| normalize(&(target - *pnt)));
                let summed_dir = normed_dirs.fold(Vec3::new(0., 0., 0.), |pnt, accum| accum + pnt);
                let target_dir = normalize(&summed_dir);

            }

        }

        let targets = targets.iter().filter(|pnt| false);
    }

    // add a little random noise to break symetry

    nodes
}


#[cfg(test)]
mod test {
    use na::{Pnt3, FloatPnt, Zero};
    use super::closest_node;

    #[test]
    fn test_closet_nodes() {
        let nodes = vec!(Pnt3::new(1.0, 2.0, 3.0), Pnt3::new(0.1, 0.1, 0.1), Pnt3::new(1.0, 1.0, 1.0));
        let point = Pnt3::new(0.0, 0.0, 0.0);
        let (closest, dist) = closest_node(&nodes, point);
        assert_eq!(closest, Pnt3::new(0.1, 0.1, 0.1));
    }
}
