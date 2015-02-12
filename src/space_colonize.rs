use na::{Pnt3, FloatPnt};
use std::cmp::Ordering::{Less, Equal, Greater};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Node {
    pos: Pnt3<f32>,
    children: Vec<Node>,
    hops: u32
}


pub fn space_colonize(source: &Vec<Pnt3<f32>>, dest: &Vec<Pnt3<f32>>, step: f32, radius: f32) -> Vec<Node> {
    let mut nodes = source.iter().map(|pnt| {
        Node{pos: pnt.clone(), children: vec!(), hops: 0}
    }).collect();

    let flat_nodes = source.clone();

    let mut targets = dest.clone();


    fn closest_node(nodes: &Vec<Pnt3<f32>>, pnt: Pnt3<f32>) -> Pnt3<f32> {
        nodes.iter()
            .map(|&base| (base, base.dist(&pnt)))
            .min_by(|&(base, dist)| base.partial_cmp(base).unwrap_or(Equal))
            .map(|(base, dist)| base)
            .unwrap()
    }

    //fn node_for_pnt


    while targets.len() > 0 {


    }

    // add a little random noise to break symetry

    nodes
}
