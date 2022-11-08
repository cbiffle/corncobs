use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut out0 = vec![0; corncobs::max_encoded_len(data.len())];
            let n = corncobs::encode_buf(data, &mut out0);

            let out1: Vec<u8> = corncobs::encode_iter(data).collect();

            assert_eq!(&out0[..n], out1);
        });
    }
}
