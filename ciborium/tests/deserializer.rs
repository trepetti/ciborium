// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

use ciborium::{Deserializer, DEFAULT_RECURSION_LIMIT, DEFAULT_SCRATCH_SIZE};

use serde::Deserialize;

use rstest::rstest;

// Reduced in scope since internal functionaly has already been tested exhaustively in codec tests.
#[rstest(expected, bytes,
    case(0u8,   "00"),
    case(1u16,  "01"),
    case(2u32,  "02"),
    case(3u64,  "03"),
    case(4u128, "04"),
    case(5i8,   "05"),
    case(6i16,  "06"),
    case(7i32,  "07"),
    case(8i64,  "08"),
    case(9i128, "09"),
    case(24u8,          "1818"),
    case(576u16,        "190240"),
    case(13824u32,      "193600"),
    case(331776u64,     "1a00051000"),
    case(7962624u128,   "1a00798000"),
    // TODO: complete.
    case([1, 2, 3], "83010203"),
)]
fn literal_deserializer<'de, T: Eq + Debug + Deserialize<'de>>(expected: T, bytes: &str) {
    let bytes = hex::decode(bytes).unwrap();

    // &[u8] implements std::io::Read and thus ciborium_io::Read.
    let slice = &bytes[..];

    let mut scratch = [0u8; DEFAULT_SCRATCH_SIZE];

    // Test Deserializer::from_reader.
    let mut deserializer_from_reader =
        Deserializer::from_reader(slice, &mut scratch, DEFAULT_RECURSION_LIMIT);
    let deserialized_from_reader: T =
        serde::de::Deserialize::deserialize(&mut deserializer_from_reader).unwrap();
    assert_eq!(deserialized_from_reader, expected);

    // Test Deserializer::from_slice.
    let mut deserializer_from_slice =
        Deserializer::from_slice(slice, &mut scratch, DEFAULT_RECURSION_LIMIT);
    let deserialized_from_slice: T =
        serde::de::Deserialize::deserialize(&mut deserializer_from_slice).unwrap();
    assert_eq!(deserialized_from_slice, expected);
}
