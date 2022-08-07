//! API tests for `corncobs`.
//!
//! These are broken into an integration test because:
//!
//! 1. The test fixtures (e.g. `#[test]`) require `std`.
//! 2. There is no way to specify that tests should run with a feature enabled.
//! 3. I don't want to include `std` in the default features because who said
//!    `std` was the default anyhow?

use corncobs::*;

static FIXTURES: &[(&[u8], &[u8])] = &[
    (&[], &[0x01, 0x00]),
    (&[0x00], &[0x01, 0x01, 0x00]),
    (&[0x00, 0x00], &[0x01, 0x01, 0x01, 0x00]),
    (&[0x11, 0x22, 0x00, 0x33], &[0x03, 0x11, 0x22, 0x02, 0x33, 0x00]),
    (&[0x11, 0x00, 0x00, 0x00], &[0x02, 0x11, 0x01, 0x01, 0x01, 0x00]),
];

#[test]
fn check_fixtures() {
    for (i, (input, output)) in FIXTURES.iter().enumerate() { 
        eprintln!("-- fixture {} --", i);
        eprintln!("input: {:x?}", input);
        eprintln!("expected: {:x?}", output);
        let mut actual = vec![0; max_encoded_len(input.len())];

        let n = encode_buf(input, &mut actual[..]);
        actual.truncate(n);

        assert_eq!(&actual[..], *output, "mismatch in test fixture case {}", i);
    }
}

#[test]
fn check_fixtures_iter() {
    for (i, (input, output)) in FIXTURES.iter().enumerate() { 
        let actual: Vec<u8> = encode_iter(input).collect();

        assert_eq!(&actual[..], *output, "mismatch in test fixture case {}", i);
    }
}

const LONG_FIXTURE_1: ([u8; 254], [u8; 254 + 2]) = {
    // Input is:
    // 01 02 ... FD FE
    let mut input = [0; 254];
    let mut i = 0;
    while i < 254 {
        input[i] = (i as u8) + 1;
        i += 1;
    }

    // Output should be:
    // FF 01 02 ... FD FE 00
    let mut output = [0; 254 + 2];
    output[0] = 0xFf;
    let mut i = 0;
    while i < 254 {
        output[i + 1] = (i as u8) + 1;
        i += 1;
    }

    (input, output)
};

const LONG_FIXTURE_2: ([u8; 255], [u8; 255 + 2]) = {
    // Input is:
    // 00 01 02 ... FD FE
    let mut input = [0; 255];
    let mut i = 0;
    while i < 255 {
        input[i] = i as u8;
        i += 1;
    }

    // Output should be:
    // 01 FF 01 02 ... FD FE 00
    let mut output = [0xDE; 255 + 2];
    output[0] = 0x01;
    output[1] = 0xFF;
    let mut i = 1;
    while i < 255 {
        output[i + 1] = i as u8;
        i += 1;
    }
    output[255 + 1] = 0;

    (input, output)
};

const LONG_FIXTURE_3: ([u8; 255], [u8; 255 + 3]) = {
    // Input is:
    // 01 02 ... FE FF
    let mut input = [0; 255];
    let mut i = 0;
    while i < 255 {
        input[i] = i as u8 + 1;
        i += 1;
    }

    // Output should be:
    // FF 01 02 ... FD FE 02 FF 00
    let mut output = [0xDE; 255 + 3];
    output[0] = 0xFF;
    let mut i = 1;
    while i < 255 {
        output[i] = i as u8;
        i += 1;
    }
    output[255] = 2;
    output[255 + 1] = 0xFF;
    output[255 + 2] = 0;

    (input, output)
};

#[test]
fn long_fixtures() {
    let fixtures: &[(&'static [u8], &'static [u8])] = &[
        (&LONG_FIXTURE_1.0, &LONG_FIXTURE_1.1),
        (&LONG_FIXTURE_2.0, &LONG_FIXTURE_2.1),
        (&LONG_FIXTURE_3.0, &LONG_FIXTURE_3.1),
    ];
    for (i, &(input, expected)) in fixtures.iter().enumerate() {
        let mut actual = vec![0; max_encoded_len(input.len())];

        let n = encode_buf(input, &mut actual[..]);
        actual.truncate(n);
        for (j, (&ab, &eb)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(ab, eb,
                "mismatch at fixture {} index {}", i, j);
        }
        assert_eq!(actual.len(), expected.len(),
        "length mismatch in fixture {}", i);

        let mut decoded = vec![0; input.len()];
        decode_buf(&actual, &mut decoded).unwrap();
        assert_eq!(&decoded, &input,
            "round-trip failed for fixture {}", i);
    }
}

#[test]
fn long_fixtures_incremental() {
    let fixtures: &[(&'static [u8], &'static [u8])] = &[
        (&LONG_FIXTURE_1.0, &LONG_FIXTURE_1.1),
        (&LONG_FIXTURE_2.0, &LONG_FIXTURE_2.1),
        (&LONG_FIXTURE_3.0, &LONG_FIXTURE_3.1),
    ];
    for (i, &(input, expected)) in fixtures.iter().enumerate() {
        println!("-- fixture {} --", i);
        let mut decoder = corncobs::Decoder::default();
        let mut input = input.iter();
        for (bi, &byte) in expected.iter().enumerate() {
            println!("{:?} <- {:x}", decoder, byte);
            match decoder.advance(byte) {
                Ok(corncobs::DecodeStatus::Append(db)) => {
                    if let Some(&next_in) = input.next() {
                        assert_eq!(db, next_in, "fixture {} idx {}", i, bi);
                    } else {
                        panic!("decode result longer than fixture");
                    }
                }
                Ok(corncobs::DecodeStatus::Pending) => (),
                Ok(corncobs::DecodeStatus::Done) => {
                    assert_eq!(input.next(), None);
                }
                Err(e) => {
                    panic!("{:?}", e);
                }
            }
        }
    }
}

#[test]
fn incremental1() {
    let mut decoder = corncobs::Decoder::default();
    let input = [4, 0x80, 0x80, 0x80, 0];
    let mut count = 0;
    for byte in input {
        match decoder.advance(byte) {
            Ok(corncobs::DecodeStatus::Append(b)) => {
                count += 1;
                assert_eq!(b, 0x80);
            }
            Ok(corncobs::DecodeStatus::Pending) => (),
            Ok(corncobs::DecodeStatus::Done) => {
                assert_eq!(count, 3);
                return;
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    panic!("did not hit done");
}

#[test]
fn long_fixtures_iter() {
    let fixtures: &[(&'static [u8], &'static [u8])] = &[
        (&LONG_FIXTURE_1.0, &LONG_FIXTURE_1.1),
        (&LONG_FIXTURE_2.0, &LONG_FIXTURE_2.1),
        (&LONG_FIXTURE_3.0, &LONG_FIXTURE_3.1),
    ];
    for (i, &(input, expected)) in fixtures.iter().enumerate() {
        let actual: Vec<u8> = encode_iter(input).collect();

        for (j, (&ab, &eb)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(ab, eb,
                "mismatch at fixture {} index {}", i, j);
        }
        assert_eq!(actual.len(), expected.len(),
        "length mismatch in fixture {}", i);

        let mut decoded = vec![0; input.len()];
        decode_buf(&actual, &mut decoded).unwrap();
        assert_eq!(&decoded, &input,
            "round-trip failed for fixture {}", i);
    }
}

#[test]
fn long_fixture_2_iter() {
    let mut input = [0; 255];
    for i in 0..255 {
        input[i] = i as u8;
    }
    // sequence is 00 01 .. FD FE
    // output should be:
    // 01 FF 01 02 ... FD FE 00
    let mut actual = vec![0; max_encoded_len(input.len())];

    let n = encode_buf(&input, &mut actual);
    actual.truncate(n);
    assert_eq!(actual.len(), input.len() + 2);

    assert_eq!(actual[0], 0x01);
    assert_eq!(actual[1], 0xFF);
    assert_eq!(actual[256], 0);
    assert_eq!(&actual[2..256], &input[1..]);

    let mut decoded = vec![0; input.len()];
    decode_buf(&actual, &mut decoded).unwrap();
    assert_eq!(&decoded, &input);
}

#[test]
fn fixture_round_trip() {
    for (i, (input, _)) in FIXTURES.iter().enumerate() { 
        let mut encoded = vec![0; max_encoded_len(input.len())];
        let n = encode_buf(input, &mut encoded);
        encoded.truncate(n);
        let mut decoded = vec![0; input.len()];
        decode_buf(&encoded, &mut decoded).unwrap();

        assert_eq!(&decoded[..], *input, "mismatch in case {}", i);
    }
}

#[test]
fn fixture_round_trip_in_place() {
    for (i, (input, _)) in FIXTURES.iter().enumerate() { 
        let mut encoded = vec![0; max_encoded_len(input.len())];
        let n = encode_buf(input, &mut encoded);
        encoded.truncate(n);

        let n = decode_in_place(&mut encoded).unwrap();

        assert_eq!(&encoded[..n], *input, "mismatch in case {}", i);
    }
}
