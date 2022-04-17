//! MQTT data representation
//!
//! <https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901006>

/// Decode a variable byte integer.
///
/// Returns `None` when there is a decoding error.
pub fn decode_variable_byte_integer(buf: &[u8]) -> Option<(u32, u8)> {
    let mut multiplier: u32 = 1;
    let mut value: u32 = 0;

    let mut buf_iter = buf.iter();
    let mut n_bytes: u8 = 0;

    loop {
        let encoded_byte: u8 = *buf_iter.next()?;
        n_bytes += 1;
        value += u32::from(encoded_byte & 0x7F) * multiplier;
        if multiplier > 128 * 128 * 128 {
            return None;
        }
        multiplier *= 128;

        if encoded_byte & 0x80 == 0 {
            break;
        }
    }

    Some((value, n_bytes))
}

pub fn encode_variable_byte_integer(mut integer: u32) -> ([u8; 4], usize) {
    let mut buf: [u8; 4] = [0; 4];
    let mut len: usize = 0;

    loop {
        buf[len] = (integer % 128) as u8;
        integer /= 128;
        if integer > 0 {
            buf[len] |= 0x80;
        }

        len += 1;

        if integer == 0 {
            break;
        }
    }

    (buf, len)
}

#[cfg(test)]
mod test {
    use super::{decode_variable_byte_integer, encode_variable_byte_integer};

    #[test]
    fn decode_variable_byte_positive_path() {
        assert_eq!(decode_variable_byte_integer(&[0x00]), Some((0, 1)));
        assert_eq!(decode_variable_byte_integer(&[0x00, 0x00]), Some((0, 1)));
        assert_eq!(decode_variable_byte_integer(&[0x7F]), Some((127, 1)));
        assert_eq!(decode_variable_byte_integer(&[0x7F, 0x00]), Some((127, 1)));
        assert_eq!(decode_variable_byte_integer(&[0x80, 0x01]), Some((128, 2)));
        assert_eq!(
            decode_variable_byte_integer(&[0xFF, 0x7F]),
            Some((16_383, 2))
        );
        assert_eq!(
            decode_variable_byte_integer(&[0x80, 0x80, 0x01]),
            Some((16_384, 3))
        );
        assert_eq!(
            decode_variable_byte_integer(&[0xFF, 0xFF, 0x7F]),
            Some((2_097_151, 3))
        );
        assert_eq!(
            decode_variable_byte_integer(&[0x80, 0x80, 0x80, 0x01]),
            Some((2_097_152, 4))
        );
        assert_eq!(
            decode_variable_byte_integer(&[0xFF, 0xFF, 0xFF, 0x7F]),
            Some((268_435_455, 4))
        );
    }

    #[test]
    fn decode_variable_byte_negative_path() {
        assert_eq!(decode_variable_byte_integer(&[]), None);
        assert_eq!(decode_variable_byte_integer(&[0x80]), None);
        assert_eq!(decode_variable_byte_integer(&[0x80, 0x80]), None);
    }

    #[test]
    fn encode_variable_byte_positive_path() {
        assert_eq!(encode_variable_byte_integer(0), ([0; 4], 1));
        assert_eq!(encode_variable_byte_integer(127), ([0x7F, 0, 0, 0], 1));
        assert_eq!(encode_variable_byte_integer(128), ([0x80, 0x01, 0, 0], 2));
        assert_eq!(
            encode_variable_byte_integer(16_383),
            ([0xFF, 0x7F, 0, 0], 2)
        );
        assert_eq!(
            encode_variable_byte_integer(16_384),
            ([0x80, 0x80, 0x01, 0], 3)
        );
        assert_eq!(
            encode_variable_byte_integer(2_097_151),
            ([0xFF, 0xFF, 0x7F, 0], 3)
        );
        assert_eq!(
            encode_variable_byte_integer(2_097_152),
            ([0x80, 0x80, 0x80, 0x01], 4)
        );
        assert_eq!(
            encode_variable_byte_integer(268_435_455),
            ([0xFF, 0xFF, 0xFF, 0x7F], 4)
        );
    }
}