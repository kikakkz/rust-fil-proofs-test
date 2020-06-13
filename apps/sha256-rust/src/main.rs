extern crate sha2;
use sha2::{Digest, Sha256};
extern crate chrono;
use chrono::prelude::*;

fn main() {
    let mut data = vec![0u8; 9728];
    for i in 0..9728 {
        data[i] = '1' as u8;
    }
    println!("HASH input {:?}", data);
    let mut dt = Local::now();
    let start = dt.timestamp_nanos();
    let hash = Sha256::digest(&data);
    dt = Local::now();
    println!("HASH result {:x?}", hash);
    println!("HASH duration {} ns", dt.timestamp_nanos() - start);
}


