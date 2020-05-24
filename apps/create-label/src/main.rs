extern crate filecoin_proofs;
extern crate storage_proofs;
extern crate rand_xorshift;
extern crate rand;
extern crate paired;

use rand::{Rng, SeedableRng};
use filecoin_proofs::*;
use rand_xorshift::XorShiftRng;
use storage_proofs::porep::stacked::StackedBucketGraph;
use crate::constants::{
    DefaultPieceHasher, DRG_DEGREE, EXP_DEGREE
};
use storage_proofs::drgraph::{new_seed, Graph};
use storage_proofs::util::NODE_SIZE;
use crate::types::DataTree;
use storage_proofs::merkle::create_base_merkle_tree;
use storage_proofs::porep::stacked::vanilla::create_label;
use storage_proofs::hasher::Sha256Domain;

extern crate chrono;
use chrono::prelude::*;

const SEED: [u8; 16] = [
    0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc, 0xe5,
];
const size: usize = 32 * 1024 * 1024;
const array: [u8; size] = [0; size];

fn main() {
    fil_logger::init();

    let mut dt = Local::now();

    println!("create DRG at {}", dt.timestamp_millis());
    let graph = StackedBucketGraph::<DefaultPieceHasher>::new_stacked(
        size / NODE_SIZE, DRG_DEGREE, EXP_DEGREE, new_seed()
    ).expect("Fail to create stackdrg");

    dt = Local::now();
    println!("create Merkle Tree at {}", dt.timestamp_millis());

    let data_tree: DataTree =
        create_base_merkle_tree::<DataTree>(None, graph.size(), &array).expect("fail");

    let layer_size = graph.size() * NODE_SIZE;
    let mut labels_buffer = vec![0u8; 2 * layer_size];
    let layer_labels = &mut labels_buffer[..layer_size];
    let replica_id = Sha256Domain::default();

    dt = Local::now();
    println!("create label at {}", dt.timestamp_millis());

    for node in 0..graph.size() {
        create_label(&graph, &replica_id, layer_labels, node);
    }

    dt = Local::now();
    println!("create done at {}", dt.timestamp_millis());
}
