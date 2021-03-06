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
use std::io::SeekFrom;
use std::io::Seek;

extern crate chrono;
use chrono::prelude::*;

const SEED: [u8; 16] = [
    0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc, 0xe5,
];

fn main() {
    fil_logger::init();

    let root_target: &str = "./";
    let cache_path: &str = &(root_target.to_owned() + "cache");

    let _root_2k: &str = "./";
    let _src_2k: &str = "./apps/data/tt";
    let _unseal_2k: &str = &(cache_path.to_owned() + "/PPPiece.ttt.2k");
    let _sealed_2k: &str = &(cache_path.to_owned() + "/PPPiece.ttt.sealed.2k");
    let _size_2k: u64 = 2032;
    let _sector_size_2k: u64 = 2048;

    let _root_32gb: &str = root_target;
    let _src_32gb: &str = "s-t0121479-0";
    let _unseal_32gb: &str = &(cache_path.to_owned() + "/PPPiece.ttt");
    let _sealed_32gb: &str = &(cache_path.to_owned() + "/PPPiece.ttt.sealed");
    let _size_32gb: u64 = 34359738368-21-270549100+2130308-16774+131;
    let _sector_size_32gb: u64 = 32 * 1024 * 1024 * 1024;

    let src: &str = _src_2k;
    let unseal: &str = _unseal_2k;
    let sealed: &str = _sealed_2k;
    let size: u64 = _size_2k;
    let sector_size: u64 = _sector_size_2k;

    let mut file = std::fs::File::open(src).expect("failed");
    file.seek(SeekFrom::Start(0)).expect("fail");
    let unsealed = std::fs::File::create(unseal).expect("failed");
    let (piece_info1, _) = filecoin_proofs::add_piece(&file, &unsealed,
                               filecoin_proofs::UnpaddedBytesAmount(size),
                               &[]).expect("failed");
    /* file.seek(SeekFrom::Start(0)).expect("fail");
    let (piece_info2, _) = filecoin_proofs::add_piece(&file, &unsealed,
                               filecoin_proofs::UnpaddedBytesAmount(size),
                               &[]).expect("failed");
    file.seek(SeekFrom::Start(0)).expect("fail");
    let (piece_info3, _) = filecoin_proofs::add_piece(&file, &unsealed,
                               filecoin_proofs::UnpaddedBytesAmount(size),
                               &[]).expect("failed");
    file.seek(SeekFrom::Start(0)).expect("fail");
    let (piece_info4, _) = filecoin_proofs::add_piece(&file, &unsealed,
                               filecoin_proofs::UnpaddedBytesAmount(size),
                               &[]).expect("failed");
    file.seek(SeekFrom::Start(0)).expect("fail");
    let (piece_info5, _) = filecoin_proofs::add_piece(&file, &unsealed,
                               filecoin_proofs::UnpaddedBytesAmount(size),
                               &[]).expect("failed");
    file.seek(SeekFrom::Start(0)).expect("fail");
    let (piece_info6, _) = filecoin_proofs::add_piece(&file, &unsealed,
                               filecoin_proofs::UnpaddedBytesAmount(size),
                               &[]).expect("failed");
    file.seek(SeekFrom::Start(0)).expect("fail");
    let (piece_info7, _) = filecoin_proofs::add_piece(&file, &unsealed,
                               filecoin_proofs::UnpaddedBytesAmount(size),
                               &[]).expect("failed");
    file.seek(SeekFrom::Start(0)).expect("fail");
    let (piece_info8, _) = filecoin_proofs::add_piece(&file, &unsealed,
                               filecoin_proofs::UnpaddedBytesAmount(size),
                               &[]).expect("failed"); */
    let piece_infos = vec![piece_info1/*, piece_info2, piece_info3, piece_info4, piece_info5, piece_info6, piece_info7, piece_info8*/];
    let _sealed = std::fs::File::create(sealed).expect("failed");

    let config = PoRepConfig {
        sector_size: SectorSize(sector_size),
        partitions: PoRepProofPartitions(
            *POREP_PARTITIONS.read().unwrap().get(&sector_size).unwrap(),
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
        cache_path,
        unseal,
        sealed,
        prover_id,
        sector_id,
        ticket,
        &piece_infos,
    ).expect("failed");

    dt = Local::now();
    println!("finish pre phase1 within {}", dt.timestamp_millis() - start);
    start = dt.timestamp_millis();

    let pre_commit_out = filecoin_proofs::seal_pre_commit_phase2(
        config, phase1_out, cache_path, sealed).expect("failed");

    let seed = rng.gen();

    let comm_d = pre_commit_out.comm_d.clone();
    let comm_r = pre_commit_out.comm_r.clone();

    dt = Local::now();
    println!("finish pre phase2 within {}", dt.timestamp_millis() - start);
    start = dt.timestamp_millis();

    let phase1_out = filecoin_proofs::seal_commit_phase1::<_, Tree>(
        config,
        cache_path,
        sealed,
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

    let sector_count = WINNING_POST_SECTOR_COUNT;
    let post_config = PoStConfig {
        sector_size: size.into(),
        sector_count,
        challenge_count: WINNING_POST_CHALLENGE_COUNT,
        typ: PoStType::Winning,
        priority: false,
    };
    // let random_fr: DefaultTreeDomain = Fr::random(rng).into();
    let randomness = [0u8; 32];
    // randomness.copy_from_slice(AsRef::<[u8]>::as_ref(&random_fr));
    let challenge_sectors = generate_winning_post_sector_challenge::<Tree>(
        &post_config,
        &randomness,
        sector_count as u64,
        prover_id,
    ).expect("fail");
    println!("Challenge sectors {:?}", challenge_sectors);
}
