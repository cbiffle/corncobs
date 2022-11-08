use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut out = vec![0; corncobs::max_encoded_len(data.len())];
            let n = corncobs::encode_buf(data, &mut out);
            let m = corncobs::decode_in_place(&mut out[..n]).unwrap();
            assert_eq!(data, &out[..m]);
        });
    }
}
