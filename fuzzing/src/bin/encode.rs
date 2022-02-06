//! Not that encoding is likely to have crashes detected by fuzz testing, but,
//! you never know.

use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut out = vec![0; corncobs::max_encoded_len(data.len())];
            corncobs::encode_buf(data, &mut out);
        });
    }
}
