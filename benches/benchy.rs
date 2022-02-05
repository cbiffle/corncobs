use criterion::{black_box, criterion_group, criterion_main, Criterion};

const RANDOM_1024: [u8; 1024] = *include_bytes!("random-1k.bin");
const ZERO_1024: [u8; 1024] = *include_bytes!("zero-1k.bin");
const FF_1024: [u8; 1024] = *include_bytes!("ff-1k.bin");

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut out = [0; corncobs::max_encoded_len(RANDOM_1024.len())];
    c.bench_function("encode_buf random 1024", move |b| b.iter(|| {
        corncobs::encode_buf(black_box(&RANDOM_1024), &mut out);
    }));

    let mut out = [0; corncobs::max_encoded_len(RANDOM_1024.len())];
    c.bench_function("encode_iter random 1024", move |b| b.iter(|| {
        for (b, o) in corncobs::encode_iter(black_box(&RANDOM_1024)).zip(&mut out) {
            *o = b;
        }
    }));


    let mut random_enc_1024 = [0; corncobs::max_encoded_len(RANDOM_1024.len())];
    let n = corncobs::encode_buf(&RANDOM_1024, &mut random_enc_1024);
    let random_enc_1024 = &random_enc_1024[..n];

    let mut out = [0; RANDOM_1024.len()];
    c.bench_function("decode_buf random 1024", move |b| b.iter(|| {
        corncobs::decode_buf(black_box(random_enc_1024), &mut out).unwrap();
    }));
    c.bench_function("decode_in_place random 1024", move |b| b.iter_batched(
        || random_enc_1024.to_vec(),
        |mut data| corncobs::decode_in_place(&mut data).unwrap(),
        criterion::BatchSize::SmallInput,
    ));


    let mut out = [0; corncobs::max_encoded_len(ZERO_1024.len())];
    c.bench_function("encode_buf zero 1024", move |b| b.iter(|| {
        corncobs::encode_buf(black_box(&ZERO_1024), &mut out);
    }));


    let mut zero_enc_1024 = [0; corncobs::max_encoded_len(ZERO_1024.len())];
    let n = corncobs::encode_buf(&ZERO_1024, &mut zero_enc_1024);
    let zero_enc_1024 = &zero_enc_1024[..n];

    let mut out = [0; ZERO_1024.len()];
    c.bench_function("decode_buf zero 1024", move |b| b.iter(|| {
        corncobs::decode_buf(black_box(zero_enc_1024), &mut out).unwrap();
    }));
    c.bench_function("decode_in_place zero 1024", move |b| b.iter_batched(
        || zero_enc_1024.to_vec(),
        |mut data| corncobs::decode_in_place(&mut data).unwrap(),
        criterion::BatchSize::SmallInput,
    ));


    let mut out = [0; corncobs::max_encoded_len(FF_1024.len())];
    c.bench_function("encode_buf ff 1024", move |b| b.iter(|| {
        corncobs::encode_buf(black_box(&FF_1024), &mut out);
    }));

    let mut ff_enc_1024 = [0; corncobs::max_encoded_len(FF_1024.len())];
    let n = corncobs::encode_buf(&FF_1024, &mut ff_enc_1024);
    let ff_enc_1024 = &ff_enc_1024[..n];

    let mut out = [0; FF_1024.len()];
    c.bench_function("decode_buf ff 1024", move |b| b.iter(|| {
        corncobs::decode_buf(black_box(ff_enc_1024), &mut out).unwrap();
    }));
    c.bench_function("decode_in_place ff 1024", move |b| b.iter_batched(
        || ff_enc_1024.to_vec(),
        |mut data| corncobs::decode_in_place(&mut data).unwrap(),
        criterion::BatchSize::SmallInput,
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
