extern crate filecoin_proofs;
extern crate storage_proofs;
extern crate rand_xorshift;
extern crate rand;
extern crate paired;

use rand::{Rng, SeedableRng};
use filecoin_proofs::*;
use rand_xorshift::XorShiftRng;
// use storage_proofs::hasher::Hasher;
// use paired::bls12_381::Fr;

const SEED: [u8; 16] = [
    0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc, 0xe5,
];

fn main() {
    fil_logger::init();

    let file = std::fs::File::open("/opt/data/source/3rd/filecoin-project/rust-fil-proofs/tt").expect("failed");
    let unsealed = std::fs::File::create("PPPiece.ttt").expect("failed");
    let (piece_info, _) = filecoin_proofs::add_piece(&file, &unsealed,
                               filecoin_proofs::UnpaddedBytesAmount(2032),
                               &[]).expect("failed");
    let piece_infos = vec![piece_info];
    let _sealed = std::fs::File::create("PPPiece.ttt.sealed").expect("failed");

    let config = PoRepConfig {
        sector_size: SectorSize(2048),
        partitions: PoRepProofPartitions(
            *POREP_PARTITIONS.read().unwrap().get(&2048).unwrap(),
        ),
    };

    let rng = &mut XorShiftRng::from_seed(SEED);
    let ticket = rng.gen();
    let sector_id = rng.gen::<u64>().into();
    let prover_id = [0u8; 32];
    // let prover_fr: <Tree::Hasher as Hasher>::Domain = Fr::random(rng).into();
    // prover_id.copy_from_slice(AsRef::<[u8]>::as_ref(&prover_fr));

    let phase1_out = filecoin_proofs::seal_pre_commit_phase1::<_, _, _, Tree>(
        config,
        "cache",
        "PPPiece.ttt",
        "PPPiece.ttt.sealed",
        prover_id,
        sector_id,
        ticket,
        &piece_infos,
    ).expect("failed");

    let pre_commit_out = filecoin_proofs::seal_pre_commit_phase2(
        config, phase1_out, "cache", "PPPiece.ttt.sealed").expect("failed");

    let seed = rng.gen();

    let comm_d = pre_commit_out.comm_d.clone();
    let comm_r = pre_commit_out.comm_r.clone();

    let phase1_out = filecoin_proofs::seal_commit_phase1::<_, Tree>(
        config,
        "cache",
        "PPPiece.ttt.sealed",
        prover_id,
        sector_id,
        ticket,
        seed,
        pre_commit_out,
        &piece_infos,
    ).expect("fail");

    println!("start phase2");
    let commit_out = filecoin_proofs::seal_commit_phase2(
        config,
        phase1_out,
        prover_id,
        sector_id,
    ).expect("fail");

    println!("start verify");
    let verified = filecoin_proofs::verify_seal::<Tree>(
        config,
        comm_r,
        comm_d,
        prover_id,
        sector_id,
        ticket,
        seed,
        &commit_out.proof,
    ).expect("fail");

    assert!(verified, "fail to verify valid seal");

    println!("hello world!");
}
