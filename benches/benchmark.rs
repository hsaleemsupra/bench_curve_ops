use std::ops::{Add, Mul};
use blsttc::{G1Projective};
use blsttc::blstrs::Scalar;
use blsttc::group::ff::Field;
use blsttc::group::Group;
use blsttc::group::prime::PrimeCurveAffine;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn benchmark_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("Blsttc");
    group.sample_size(10);

    let configs = vec![10,100,1000,10000];

    for config in &configs {

        let g1_rand = G1Projective::random(&mut rand::thread_rng());
        let g1_rand_2 = G1Projective::random(&mut rand::thread_rng());
        let s = Scalar::random(&mut rand::thread_rng());

        group.bench_with_input(BenchmarkId::new("Blsttc: Curve Add", format!("n: {}", config)), &config, |b, _cfg| {
            b.iter(|| {
                for _i in 0..*config{
                    let add_res = g1_rand.add(g1_rand_2);
                    black_box(add_res);
                }
            });

        });

        group.bench_with_input(BenchmarkId::new("Blsttc: Curve Mul", format!("n: {}", config)), &config, |b, _cfg| {
            b.iter(|| {
                for _i in 0..*config{
                    let rand_g1 = g1_rand.mul(&s);
                    black_box(rand_g1);
                }

            });
        });

        let mut points = Vec::new();
        let mut s_vec = Vec::new();

        for _i in 0..*config{
            let rand_g1 = G1Projective::random(&mut rand::thread_rng());
            points.push(rand_g1);

            let s = Scalar::random(&mut rand::thread_rng());
            s_vec.push(s);
        }

        let mut naive = points[0] * s_vec[0];
        for i in 1..*config {
            naive += points[i] * s_vec[i];
        }

        let pippenger = G1Projective::multi_exp(&points, &s_vec);
        assert_eq!(naive, pippenger);

        group.bench_with_input(BenchmarkId::new("Blsttc: Curve MSM", format!("n: {}", config)), &config, |b, _cfg| {
            b.iter(|| {
                let mul_res = G1Projective::multi_exp(&points, &s_vec);
                black_box(mul_res);
            });

        });

    }
    group.finish();
}


criterion_group!(benches, benchmark_config);
criterion_main!(benches);