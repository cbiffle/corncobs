static FIXTURES: &[(&[u8], &[u8])] = &[
    (&[], &[0x01, 0x00]),
    (&[0x00], &[0x01, 0x01, 0x00]),
    (&[0x00, 0x00], &[0x01, 0x01, 0x01, 0x00]),
    (&[0x11, 0x22, 0x00, 0x33], &[0x03, 0x11, 0x22, 0x02, 0x33, 0x00]),
    (&[0x11, 0x00, 0x00, 0x00], &[0x02, 0x11, 0x01, 0x01, 0x01, 0x00]),
];

const RANDOM_1024: [u8; 1024] = *include_bytes!("../benches/random-1k.bin");
const ZERO_1024: [u8; 1024] = *include_bytes!("../benches/zero-1k.bin");
const FF_1024: [u8; 1024] = *include_bytes!("../benches/ff-1k.bin");

#[test]
fn check_cobs_rs() {
    for (i, (input, _output)) in FIXTURES.iter().enumerate() { 
        eprintln!("-- fixture {} --", i);
        let mut actual = vec![0; corncobs::max_encoded_len(input.len())];

        let cclen = corncobs::encode_buf(input, &mut actual);
        let ccout = &actual[..cclen];
        eprintln!("corncobs: {:x?}", ccout);

        // cobs_rs wants to know input length at compile time. Seriously. It
        // takes the length of the message at compile time.
        //
        // Sigh.
        let crout: [u8; 6];
        match input.len() {
            0 => {
                let mut ary: [u8; 0] = (*input).try_into().unwrap();
                crout = cobs_rs::stuff(ary, corncobs::ZERO);
            }
            1 => {
                let mut ary: [u8; 1] = (*input).try_into().unwrap();
                crout = cobs_rs::stuff(ary, corncobs::ZERO);
            }
            2 => {
                let mut ary: [u8; 2] = (*input).try_into().unwrap();
                crout = cobs_rs::stuff(ary, corncobs::ZERO);
            }
            4 => {
                let mut ary: [u8; 4] = (*input).try_into().unwrap();
                crout = cobs_rs::stuff(ary, corncobs::ZERO);
            }
            _ => panic!("need to hardcode another length"),
        }
        eprintln!("cobs_rs: {:x?}", crout);

        for (j, (ours, theirs)) in ccout.iter().zip(&crout).enumerate() {
            assert_eq!(theirs, ours, "mismatch at fixture {} index {}", i, j);
        }

        // cobs_rs doesn't return the length of the output, we have to find it
        // with a scan.
        let crlen = crout.iter().position(|&b| b == corncobs::ZERO)
            .unwrap() + 1;
        assert_eq!(crlen, ccout.len(), "length mismatch at fixture {}", i);
    }
}

#[test]
fn check_postcard_cobs() {
    for (i, (input, _output)) in FIXTURES.iter().enumerate() { 
        let skips = [0];
        if skips.contains(&i) {
            eprintln!("skipping known-broken fixture {}", i);
            continue;
        }
        eprintln!("-- fixture {} --", i);
        let mut actual = vec![0; corncobs::max_encoded_len(input.len())];

        let cclen = corncobs::encode_buf(input, &mut actual);
        let ccout = &actual[..cclen];
        eprintln!("corncobs: {:x?}", ccout);

        let mut actual = vec![0; corncobs::max_encoded_len(input.len())];
        let pclen = postcard_cobs::encode(input, &mut actual);
        let pcout = &actual[..pclen];
        eprintln!("postcard: {:x?}", pcout);

        for (j, (ours, theirs)) in ccout.iter().zip(pcout).enumerate() {
            assert_eq!(theirs, ours, "mismatch at fixture {} index {}", i, j);
        }

        // Postcard does not zero-terminate.
        assert_eq!(pcout.len(), ccout.len() - 1, "length mismatch at fixture {}", i);
    }
}
