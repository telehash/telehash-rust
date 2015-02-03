
use std::string;
use std::iter;

static STANDARD_CHARS: &'static[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

pub fn encode(b: &[u8]) -> Result<String, string::FromUtf8Error> {
    let mut out = Vec::new();
    let mut n: u64 = 0;

    for i in 0 .. b.len() {
        // Calculate the shift (32, 24, 16, 8, 0)
        let shift = 8 * (4 - (i % 5));
        n |= (b[i] as u64) << shift;

        // If the shift was zero, or this is the last character of input, extract
        // each 5-bit group from n.
        if shift == 0 || (i == b.len()-1) {
            // Determine number of output groups, based on the shift
            let groups = match shift {
                32 => 2,
                24 => 4,
                16 => 5,
                8  => 7,
                _  => 8
            };

            // Pull out each 5-bit index
            for j in iter::range_step(35, (35 - (groups * 5)), -5) {
                let ndx = (n >> j as u32) & 31;
                out.push(STANDARD_CHARS[ndx as usize]);
            }

            // Reset n
            n = 0;
        }
    }

    String::from_utf8(out)
}

pub fn decode(b: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut n: u64 = 0;

    for i in range(0, b.len()) {
        // Use the character to generate an index
        let c = b[i];
        let idx = match c as char {
            'A' ... 'Z' => c - 65, // A..Z => 65..90
            _ => 26 + (c - 50)     // 2..7 => 50..55
        };

        // Calculate the shift for this 5-bit group (35, 25, ..., 0)
        let shift = 35 - (5 * (i % 8));
        n |= (idx as u64 & 31) << shift;

        // If the shift is zero or this is the last character of input, extract
        // each 8-bit group from n.
        if shift == 0 || (i == b.len()-1) {
            // Determine number of output groups, based on shift
            let groups = match shift {
                35 => 0,
                30 => 1,
                25 => 2,
                20 => 2,
                15 => 3,
                10 => 4,
                5  => 4,
                _  => 5
            };

            // Pull out each 8-bit group
            for j in iter::range_step(32, (32 - (groups * 8)), -8) {
                let v = ((n >> j as u8) & 255) as u8;
                out.push(v);
            }

            // Reset n
            n = 0;
        }
    }

    out
}

macro_rules! check_encode(
    ($input:expr, $expected:expr) => ( {
        let x: Result<String, string::FromUtf8Error> = encode($input.as_bytes());
        assert_eq!(x.is_ok(), true);
        assert_eq!($expected, x.unwrap());
    });
    );

macro_rules! check_decode(
    ($expected:expr, $input:expr) => (
        assert_eq!($expected.as_bytes(), decode($input.as_bytes()));
        );
);



#[test]
fn encode_tests() {
    check_encode!("", "");
    check_encode!("f", "MY");
    check_encode!("foo", "MZXW6");
    check_encode!("foob", "MZXW6YQ");
    check_encode!("fooba", "MZXW6YTB");
    check_encode!("foobar", "MZXW6YTBOI");
}

#[test]
fn decode_tests() {
    check_decode!("", "");
    check_decode!("f", "MY");
    check_decode!("foo", "MZXW6");
    check_decode!("foob", "MZXW6YQ");
    check_decode!("fooba", "MZXW6YTB");
    check_decode!("foobar", "MZXW6YTBOI");
}

