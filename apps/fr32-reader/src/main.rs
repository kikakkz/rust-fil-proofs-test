extern crate filecoin_proofs;
extern crate storage_proofs;
extern crate rand_xorshift;
extern crate rand;
extern crate paired;

extern crate chrono;
// use chrono::prelude::*;

use filecoin_proofs::fr32_reader;
use std::io::Read;

fn main() {
    fil_logger::init();
    let file = std::fs::File::open("./apps/data/fr32.bin").expect("failed");
    let source = std::io::BufReader::new(&file);
    let mut fr32_reader = fr32_reader::Fr32Reader::new(source);
    let mut buf = vec![0; 512];
    loop {
        match fr32_reader.read(&mut buf) {
            Ok(rlen) => {
                if 0 == rlen {
                    break;
                }
                println!("Read {} bytes", rlen);
            },
            Err(_e) => {
                println!("Done");
                break;
            },
        }
    }

    println!("{:?}", buf);
}
