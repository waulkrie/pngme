// https://picklenerd.github.io/pngme_book/chapter_1.html

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    chunk_type_code: u32,
}
impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.chunk_type_code.to_be_bytes()
    }
    fn is_valid(&self) -> bool {
        for elem in self.chunk_type_code.to_be_bytes() {
            if !elem.is_ascii_alphabetic() {
                return false;
            }
        }
        true
    }

    // Ancillary bit: bit 5 of first byte
    //      0 (uppercase) = critical, 1 (lowercase) = ancillary.
    fn is_critical(&self) -> bool {
        self.bytes()[0] & 0b0001_0000 == 0
    }

    //Private bit: bit 5 of second byte
    //     0 (uppercase) = public, 1 (lowercase) = private.
    fn is_public(&self) -> bool {
        self.bytes()[1] & 0b0001_0000 == 0
    }
    //Reserved bit: bit 5 of third byte
    //     Must be 0 (uppercase) in files conforming to this version of PNG.
    fn is_reserved_bit_valid(&self) -> bool {
        self.bytes()[2] & 0b0001_0000 == 0
    }
    //Safe-to-copy bit: bit 5 of fourth byte
    //     0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
    fn is_safe_to_copy(&self) -> bool {
        self.bytes()[3] & 0b0001_0000 == 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let mut ret: ChunkType = ChunkType { chunk_type_code: 0 };
        ret.chunk_type_code = u32::from_be_bytes(value);
        Ok(ret)
    }
}
impl FromStr for ChunkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret: ChunkType = ChunkType { chunk_type_code: 0 };
        let mut bytes: [u8; 4] = [0, 0, 0, 0];
        //
        //for i in 0..4{
        //    bytes[i] = s.as_bytes()[i];
        //}
        bytes[..4].copy_from_slice(&s.as_bytes()[..4]);
        ret.chunk_type_code = u32::from_be_bytes(bytes);
        if ret.is_valid() {
            Ok(ret)
        } else {
            Err(())
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let chars: Vec<char> = (0..32)
            .step_by(8)
            .map(|i| {
                let shift = i;
                ((self.chunk_type_code >> shift) & 0xff) as u8 as char
            })
            .collect();
        let string: String = chars.into_iter().rev().collect();
        write!(f, "{}", string)
    }
}

// Unit Tests

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
