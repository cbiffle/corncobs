use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut out = data.to_vec();
            corncobs::decode_buf(data, &mut out).ok();
        });
    }
}
