#[macro_use]
extern crate criterion;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate lab;

use lab::Lab;
use criterion::Criterion;
use rand::Rng;
use rand::distributions::Standard;

lazy_static! {
    static ref RGBS: Vec<[u8;3]> = {
        let rand_seed = [0u8;32];
        let mut rng: rand::StdRng = rand::SeedableRng::from_seed(rand_seed);
        rng.sample_iter(&Standard).take(512).collect()
    };
}

fn rgb_to_lab(c: &mut Criterion) {
    let mut rgbs = RGBS.iter().cycle();
    c.bench_function("rgb_to_lab", move |b| {
        b.iter(|| Lab::from_rgb(&rgbs.next().unwrap()))
    });
}

fn rgbs_to_labs(c: &mut Criterion) {
    c.bench_function("rgbs_to_labs", move |b| {
        b.iter(|| Lab::from_rgbs(&RGBS))
    });
}

fn rgbs_to_labs_avx(c: &mut Criterion) {
    c.bench_function("rgbs_to_labs_avx", move |b| {
        b.iter(|| Lab::from_rgbs_avx(&RGBS))
    });
}

criterion_group!(benches, rgb_to_lab, rgbs_to_labs, rgbs_to_labs_avx);
criterion_main!(benches);
