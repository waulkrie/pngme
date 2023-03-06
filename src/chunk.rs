use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_CKSUM};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// The first eight bytes of a PNG file always contain the following (decimal) values:
// 137 80 78 71 13 10 26 10
struct Chunk {
    len: u32,
    chuck_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}
impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk {
            len: 0,
            chuck_type: ChunkType::from_str("RuSt").unwrap(),
            chunk_data: vec![],
            crc: 0,
        }
    }
    fn length(&self) -> u32 {
        self.len
    }
    fn chunk_type(&self) -> &ChunkType {
        &self.chuck_type
    }
    fn data(&self) -> &[u8] {
        self.chunk_data.as_slice()
    }
    fn crc(&self) -> u32 {
        self.crc
    }

    fn data_as_string(&self) -> Result<String, String> {
        let ret: String = self.chunk_data.iter().map(|x| *x as char).collect();
        return Ok(ret);
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.chunk_data.to_vec()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // println!("value {:?}", String::from_utf8(value.to_vec()));
        const CHECK_U32: Crc<u32> = Crc::<u32>::new(&CRC_32_CKSUM);
        let slice_end = value.len() - 4;
        let val_str: String = value[0..slice_end].iter().map(|x| *x as char).collect();
        println!("val_str {}", val_str);
        let mut bytes: [u8; 4] = [0, 0, 0, 0];
        //
        //for i in 0..4{
        //    bytes[i] = s.as_bytes()[i];
        //}
        bytes[..4].copy_from_slice(&value[..4]);
        let ret: Chunk = Chunk {
            len: u32::from_be_bytes(bytes),
            chuck_type: ChunkType::from_str(&val_str[4..8]).unwrap(),
            chunk_data: value[8..].to_vec(),
            crc: CHECK_U32.checksum(value),
        };

        Ok(ret)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
