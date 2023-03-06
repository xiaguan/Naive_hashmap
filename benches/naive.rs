#[macro_use]
extern crate criterion;
extern crate naive_hashmap;
extern crate rand;

use criterion::{BenchmarkId, Criterion};
use rand::Rng;

fn insert_and_lookup_naive(n: u64) {
    let mut rng = rand::thread_rng();
    let mut hash_map = naive_hashmap::HashMap::new();

    for _ in 0..n {
        let key = rng.gen::<u8>();
        if rng.gen::<bool>() {
            let value = rng.gen::<u32>();
            hash_map.insert(key, value);
        } else {
            hash_map.lookup(key);
        }
    }
}

fn insert_and_lookup_standard(n: u64) {
    let mut rng = rand::thread_rng();
    let mut hash_map = std::collections::HashMap::new();

    for _ in 0..n {
        let key = rng.gen::<u8>();
        if rng.gen::<bool>() {
            let value = rng.gen::<u32>();
            hash_map.insert(key, value);
        } else {
            hash_map.get(&key);
        }
    }
}

fn bench_insert_and_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_and_lookup");
    for i in [1, 100, 1000, 10000, 100000].iter() {
        group.bench_with_input(BenchmarkId::new("naive", i), i, |b, i| {
            b.iter(|| insert_and_lookup_naive(*i))
        });
        group.bench_with_input(BenchmarkId::new("standard", i), i, |b, i| {
            b.iter(|| insert_and_lookup_standard(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_insert_and_lookup);
criterion_main!(benches);
