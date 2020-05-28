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

extern crate chrono;
use chrono::prelude::*;

const SEED: [u8; 16] = [
    0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc, 0xe5,
];

fn main() {
    fil_logger::init();

	let ROOT_TARGET: &str = "/home/yt/test/";
	let CACHE_PATH: &str = &(ROOT_TARGET.to_owned() + "cache");

	let ROOT_2K: &str = "./";
	let SRC_2K: &str = "tt";
	let UNSEAL_2K: &str = &(ROOT_TARGET.to_owned() + "PPPiece.ttt.2k");
	let SEALED_2K: &str = &(ROOT_TARGET.to_owned() + "PPPiece.ttt.sealed.2k");
	let SIZE_2K: u64 = 2032;
	let SECTOR_SIZE_2K: u64 = 2048;

	let ROOT_32GB: &str = ROOT_TARGET;
	let SRC_32GB: &str = "s-t0121479-0";
	let UNSEAL_32GB: &str = &(ROOT_TARGET.to_owned() + "PPPiece.ttt");
	let SEALED_32GB: &str = &(ROOT_TARGET.to_owned() + "PPPiece.ttt.sealed");
	let SIZE_32GB: u64 = 34359738368-21-270549100+2130308-16774+131;
	let SECTOR_SIZE_32GB: u64 = 32 * 1024 * 1024 * 1024;

	let ROOT: &str = ROOT_2K;
	let SRC: &str = SRC_2K;
	let UNSEAL: &str = UNSEAL_2K;
	let SEALED: &str = SEALED_2K;
	let SIZE: u64 = SIZE_2K;
	let SECTOR_SIZE: u64 = SECTOR_SIZE_2K;

    let file = std::fs::File::open(SRC).expect("failed");
    let unsealed = std::fs::File::create(UNSEAL).expect("failed");
    let (piece_info, _) = filecoin_proofs::add_piece(&file, &unsealed,
                               filecoin_proofs::UnpaddedBytesAmount(SIZE),
                               &[]).expect("failed");
    let piece_infos = vec![piece_info];
    let _sealed = std::fs::File::create(SEALED).expect("failed");

    let config = PoRepConfig {
        sector_size: SectorSize(SECTOR_SIZE),
        partitions: PoRepProofPartitions(
            *POREP_PARTITIONS.read().unwrap().get(&SECTOR_SIZE).unwrap(),
        ),
    };

    let rng = &mut XorShiftRng::from_seed(SEED);
    let ticket = rng.gen();
    let sector_id = rng.gen::<u64>().into();
    let prover_id = [0u8; 32];
    // let prover_fr: <Tree::Hasher as Hasher>::Domain = Fr::random(rng).into();
    // prover_id.copy_from_slice(AsRef::<[u8]>::as_ref(&prover_fr));

    let mut dt = Local::now();
	let mut start = dt.timestamp_millis();
    println!("start pre phase1 at {}", dt.timestamp_millis());

    let phase1_out = filecoin_proofs::seal_pre_commit_phase1::<_, _, _, Tree>(
        config,
        CACHE_PATH,
		UNSEAL,
		SEALED,
        prover_id,
        sector_id,
        ticket,
        &piece_infos,
    ).expect("failed");

    dt = Local::now();
    println!("finish pre phase1 within {}", dt.timestamp_millis() - start);
	start = dt.timestamp_millis();

    let pre_commit_out = filecoin_proofs::seal_pre_commit_phase2(
        config, phase1_out, CACHE_PATH, SEALED).expect("failed");

    let seed = rng.gen();

    let comm_d = pre_commit_out.comm_d.clone();
    let comm_r = pre_commit_out.comm_r.clone();

    dt = Local::now();
    println!("finish pre phase2 within {}", dt.timestamp_millis() - start);
	start = dt.timestamp_millis();

    let phase1_out = filecoin_proofs::seal_commit_phase1::<_, Tree>(
        config,
		CACHE_PATH,
		SEALED,
        prover_id,
        sector_id,
        ticket,
        seed,
        pre_commit_out,
        &piece_infos,
    ).expect("fail");

    dt = Local::now();
    println!("finish commit phase1 within {}", dt.timestamp_millis() - start);
	start = dt.timestamp_millis();

    let commit_out = filecoin_proofs::seal_commit_phase2(
        config,
        phase1_out,
        prover_id,
        sector_id,
    ).expect("fail");

    dt = Local::now();
    println!("finish commit phase2 within {}", dt.timestamp_millis() - start);
	start = dt.timestamp_millis();

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
    dt = Local::now();
    println!("finish verify within {}", dt.timestamp_millis() - start);
}
