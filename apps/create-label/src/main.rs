extern crate filecoin_proofs;
extern crate storage_proofs;
extern crate rand_xorshift;
extern crate rand;
extern crate paired;

use filecoin_proofs::*;
use storage_proofs::porep::stacked::StackedBucketGraph;
use crate::constants::{
    DefaultPieceHasher, DRG_DEGREE, EXP_DEGREE
};
use storage_proofs::drgraph::{new_seed, Graph};
use storage_proofs::util::NODE_SIZE;
use crate::types::DataTree;
use storage_proofs::merkle::create_base_merkle_tree;
use storage_proofs::porep::stacked::vanilla::{create_label, create_label_exp};
use storage_proofs::hasher::Sha256Domain;

extern crate chrono;
use chrono::prelude::*;

const SIZE: usize = 1024 * 1024;
const LAYERS: i8 = 2;

fn main() {
    fil_logger::init();

    let mut dt = Local::now();
    let mut start = dt.timestamp_millis();
    let array = vec![0; SIZE];

    println!("create DRG at {} with size {}", dt.timestamp_millis(), SIZE);
    let graph = StackedBucketGraph::<DefaultPieceHasher>::new_stacked(
        SIZE / NODE_SIZE, DRG_DEGREE, EXP_DEGREE, new_seed()
    ).expect("Fail to create stackdrg");

    dt = Local::now();
    println!("Construct graph within {} ms graph size {}", dt.timestamp_millis() - start, graph.size());
    start = dt.timestamp_millis();

    let _data_tree: DataTree =
        create_base_merkle_tree::<DataTree>(None, graph.size(), &array).expect("fail");

    let layer_size = graph.size() * NODE_SIZE;
    let mut labels_buffer = vec![0u8; 2 * layer_size];
    let replica_id = Sha256Domain::default();

    dt = Local::now();
    println!("Create merkle tree within {} ms", dt.timestamp_millis() - start);
    start = dt.timestamp_millis();
    let label_start = start;

    let layers = LAYERS;
    println!("Create Layer at {} nodes {}", dt.timestamp_millis(), graph.size());

    for layer in 1..=layers {
        if 1 == layer {
            let layer_labels = &mut labels_buffer[..layer_size];
            for node in 0..graph.size() {
                create_label(&graph, &replica_id, layer_labels, node).expect("fail");
            }
        } else {
            let (layer_labels, exp_labels) = labels_buffer.split_at_mut(layer_size);
            for node in 0..graph.size() {
                create_label_exp(&graph, &replica_id, exp_labels, layer_labels, node).expect("fail");
            }
        }
        dt = Local::now();
        println!("Create layer {} within {} ms", layer, dt.timestamp_millis() - start);
        start = dt.timestamp_millis();
    }

    dt = Local::now();
    println!("Create layers within {} ms", dt.timestamp_millis() - label_start);
    println!("Create done at {}", dt.timestamp_millis());
}
