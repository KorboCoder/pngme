use std::fmt::Display;

use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};


pub const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

struct Chunk{
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32
}

impl Chunk {

    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mut to_crc = vec!(chunk_type.0, chunk_type.1, chunk_type.2, chunk_type.3);
        to_crc.append(&mut data.clone());
        Chunk {
            length: data.len() as u32,
            chunk_type,
            chunk_data: data,
            crc: CRC.checksum(to_crc.as_slice())
        }
    }

    fn length(&self) -> u32{
        self.length
    }

    fn chunk_type(&self) -> &ChunkType{
        &(self.chunk_type)
    }

    fn data(&self) -> &[u8]{
        self.chunk_data.as_slice()
    }

    fn data_as_string(&self) -> Result<String, ()> {
        Ok(String::from_utf8(self.chunk_data.clone()).unwrap())
    }

    fn crc(&self) -> u32 {
        self.crc
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut fixed_length: [u8; 4] = [0; 4];
        fixed_length.clone_from_slice(&value[0..4]);

        let length:u32 = u32::from_be_bytes(fixed_length);


        let mut raw_chunk: [u8; 4] = [0; 4];
        raw_chunk.clone_from_slice(&value[4..8]);
        let chunk_type = ChunkType::try_from(raw_chunk);

        if let Ok(chunk_type) = chunk_type {

            if(!chunk_type.is_valid()){
                return Err(());
            }

            let mut chunk_data:Vec<u8> = vec!();
            chunk_data.extend_from_slice(&value[8..(8+(length as usize))]);


            let actual_crc: u32 = CRC.checksum(&value[4..(8+length as usize)]);

            let mut fixed_crc: [u8; 4] = [0; 4];
            fixed_crc.clone_from_slice(&value[(8+length as usize)..]);
            let  expected_crc: u32 = u32::from_be_bytes(fixed_crc);

            if(actual_crc != expected_crc){
                return Err(());
            }

            return Ok(Chunk {
                length,
                chunk_type,
                chunk_data,
                crc: actual_crc
            });
        }
        else  {
            return Err(());
        }
        // let chunk_data = value.copy_within(6..(6+len), len as usize);

    }
}

impl Display for Chunk{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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
