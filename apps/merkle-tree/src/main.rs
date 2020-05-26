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

extern crate chrono;
use chrono::prelude::*;

const SIZE: usize = 1 * 1024 * 1024;

fn main() {
    fil_logger::init();

    let mut dt = Local::now();
    let mut start = dt.timestamp_millis();
    let array = vec![0; SIZE];

    println!("create DRG at {}", dt.timestamp_millis());
    let graph = StackedBucketGraph::<DefaultPieceHasher>::new_stacked(
        SIZE / NODE_SIZE, DRG_DEGREE, EXP_DEGREE, new_seed()
    ).expect("Fail to create stackdrg");

    dt = Local::now();
    println!("Construct graph within {} ms", dt.timestamp_millis() - start);
    start = dt.timestamp_millis();

    let _data_tree: DataTree =
        create_base_merkle_tree::<DataTree>(None, graph.size(), &array).expect("fail");

    dt = Local::now();
    println!("Create merkle tree within {} ms", dt.timestamp_millis() - start);
}
