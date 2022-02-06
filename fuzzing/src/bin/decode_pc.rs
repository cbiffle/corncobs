//! Fuzz test for postcard-cobs, to check whether I was simply holding it wrong
//! or if it really does panic freely.

use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut out = data.to_vec();
            postcard_cobs::decode(data, &mut out).ok();
        });
    }
}
