#[macro_use]
extern crate criterion;

use merkletree::merkle::MerkleTree;
use merkletree::streaming::MerkleStreamer;

use criterion::{black_box, Criterion, ParameterizedBenchmark};
use rand::{thread_rng, Rng};
use std::io::Cursor;
use storage_proofs::drgraph::{new_seed, Graph};
use storage_proofs::hasher::blake2s::{Blake2sDomain, Blake2sFunction, Blake2sHasher};
use storage_proofs::hasher::pedersen::{PedersenDomain, PedersenFunction, PedersenHasher};
use storage_proofs::zigzag_graph::{ZigZag, ZigZagBucketGraph, DEFAULT_EXPANSION_DEGREE};

fn merkle_benchmark(c: &mut Criterion) {
    #[cfg(feature = "big-sector-sizes-bench")]
    let params = vec![128, 1024, 1048576];
    #[cfg(not(feature = "big-sector-sizes-bench"))]
    let params = vec![128, 1024];

    c.bench(
        "merkletree",
        ParameterizedBenchmark::new(
            "blake2s",
            move |b, nodes| {
                let mut rng = thread_rng();
                let data: Vec<u8> = (0..32 * *nodes).map(|_| rng.gen()).collect();
                let graph = ZigZagBucketGraph::<Blake2sHasher>::new_zigzag(
                    *nodes,                   // #nodes
                    8,                        // degree
                    DEFAULT_EXPANSION_DEGREE, // expansion degree,
                    new_seed(),
                );

                b.iter(|| black_box(graph.merkle_tree(&data).unwrap()))
            },
            params,
        )
        .with_function("pedersen", move |b, nodes| {
            let mut rng = thread_rng();
            let data: Vec<u8> = (0..32 * *nodes).map(|_| rng.gen()).collect();

            let graph = ZigZagBucketGraph::<PedersenHasher>::new_zigzag(
                *nodes,                   // #nodes
                8,                        // degree
                DEFAULT_EXPANSION_DEGREE, // expansion degree,
                new_seed(),
            );

            b.iter(|| black_box(graph.merkle_tree(&data).unwrap()))
        })
        .with_function("pedersen streaming", move |b, nodes| {
            let mut rng = thread_rng();
            let data: Vec<PedersenDomain> = (0..*nodes).map(|_| rng.gen()).collect();

            let out = Cursor::new(Vec::with_capacity((data.len() - 1) * 512));
            b.iter(|| {
                black_box(
                    MerkleStreamer::<PedersenDomain, PedersenFunction, _>::from_iter(
                        data.clone(),
                        out.clone(),
                    ),
                )
            });
        })
        .with_function("blake streaming", move |b, nodes| {
            let mut rng = thread_rng();
            let data: Vec<Blake2sDomain> = (0..*nodes).map(|_| rng.gen()).collect();

            let out = Cursor::new(Vec::with_capacity((data.len() - 1) * 512));
            b.iter(|| {
                black_box(
                    MerkleStreamer::<Blake2sDomain, Blake2sFunction, _>::from_iter(
                        data.clone(),
                        out.clone(),
                    ),
                )
            });
        })
        .sample_size(20),
    );
}

criterion_group!(benches, merkle_benchmark);
criterion_main!(benches);
