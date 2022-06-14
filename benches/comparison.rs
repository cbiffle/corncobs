use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

const RANDOM_1024: [u8; 1024] = *include_bytes!("random-1k.bin");
const ZERO_1024: [u8; 1024] = *include_bytes!("zero-1k.bin");
const FF_1024: [u8; 1024] = *include_bytes!("ff-1k.bin");

const STIMULI: [(&str, &[u8; 1024]); 3] = [
    ("ff", &FF_1024),
    ("random", &RANDOM_1024),
    ("zero", &ZERO_1024),
];

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_cross");

    for (set, data) in STIMULI {
        const FIXED_LEN: usize = 1024;
        assert_eq!(data.len(), FIXED_LEN);

        let mut out = [0; corncobs::max_encoded_len(FIXED_LEN)];
        group.bench_with_input(BenchmarkId::new("corncobs", set), data, move |b, i| {
            b.iter(|| corncobs::encode_buf(i, &mut out));
        });

        let mut out = [0; corncobs::max_encoded_len(FIXED_LEN)];
        group.bench_with_input(BenchmarkId::new("cobs", set), data, move |b, i| {
            b.iter(|| cobs::encode(i, &mut out));
        });

        const FIXED_OUT_LEN: usize = corncobs::max_encoded_len(1024);
        group.bench_with_input(BenchmarkId::new("cobs-rs", set), data, move |b, i| {
            b.iter(|| cobs_rs::stuff::<1024, FIXED_OUT_LEN>(*i, corncobs::ZERO));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("decode_cross");

    for (set, data) in STIMULI {
        const FIXED_LEN: usize = 1024;
        assert_eq!(data.len(), FIXED_LEN);

        const ELEN: usize = corncobs::max_encoded_len(FIXED_LEN);
        let mut encoded = [0; ELEN];
        let _ = corncobs::encode_buf(data, &mut encoded);

        let mut out = [0; FIXED_LEN];
        group.bench_with_input(BenchmarkId::new("corncobs", set), &encoded, move |b, i| {
            b.iter(|| corncobs::decode_buf(i, &mut out));
        });

        group.bench_with_input(BenchmarkId::new("cobs-rs", set), &encoded, move |b, i| {
            b.iter(|| cobs_rs::unstuff::<ELEN, FIXED_LEN>(*i, corncobs::ZERO));
        });

        let mut out = [0; FIXED_LEN];
        group.bench_with_input(BenchmarkId::new("cobs", set), &encoded, move |b, i| {
            b.iter(|| cobs::decode(i, &mut out));
        });

    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
