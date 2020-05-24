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

const SIZE: usize = 2 * 1024;
const ARRAY: [u8; SIZE] = [0; SIZE];
const LAYERS: i8 = 1;

fn main() {
    fil_logger::init();

    let mut dt = Local::now();
    let mut start = dt.timestamp_millis();

    println!("create DRG at {}", dt.timestamp_millis());
    let graph = StackedBucketGraph::<DefaultPieceHasher>::new_stacked(
        SIZE / NODE_SIZE, DRG_DEGREE, EXP_DEGREE, new_seed()
    ).expect("Fail to create stackdrg");

    dt = Local::now();
    println!("Construct graph within {}", dt.timestamp_millis() - start);
    start = dt.timestamp_millis();

    let _data_tree: DataTree =
        create_base_merkle_tree::<DataTree>(None, graph.size(), &ARRAY).expect("fail");

    let layer_size = graph.size() * NODE_SIZE;
    let mut labels_buffer = vec![0u8; 2 * layer_size];
    let replica_id = Sha256Domain::default();

    dt = Local::now();
    println!("Create merkle tree within {}", dt.timestamp_millis() - start);
    start = dt.timestamp_millis();
    let label_start = start;

    let layers = LAYERS;
    println!("create Layer at {}", dt.timestamp_millis());

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
        println!("Create layer {} within {}", layer, dt.timestamp_millis() - start);
        start = dt.timestamp_millis();
    }

    dt = Local::now();
    println!("Create layers within {}", dt.timestamp_millis() - label_start);
    println!("Create done at {}", dt.timestamp_millis());
}
