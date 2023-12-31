use std::{fmt::Display, string::FromUtf8Error, io::{BufReader, Read}};

use crate::chunk_type::{ChunkType, ChunkTypeError};
use crc::{Crc, CRC_32_ISO_HDLC};
use thiserror::Error;

pub const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);


#[derive(Debug, Error)]
pub enum ChunkError {
    #[error("Error reading length bytes")]
    LengthByteRead,
    #[error("Error reading Chunk Type bytes")]
    ChunkTypeByteRead,
    #[error("Error reading Data bytes")]
    DataByteRead,
    #[error("Error reading Crc bytes")]
    CrcByteRead,
    #[error("Crc does not match.")]
    CrcMismatch,
    #[error("Invalid Chunk Type: {0}")]
    ChunkTypeError(ChunkTypeError)
}

pub struct Chunk{
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32
}

impl Chunk {

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {

        let to_crc: Vec<u8> = vec!(chunk_type.0, chunk_type.1, chunk_type.2, chunk_type.3)
            .iter()
            .cloned()
            .chain(data.iter().cloned())
            .collect();

        Chunk {
            length: data.len() as u32,
            chunk_type,
            chunk_data: data,
            crc: CRC.checksum(to_crc.as_slice())
        }

    }

    pub fn length(&self) -> u32{
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType{
        &(self.chunk_type)
    }

    pub fn data(&self) -> &[u8]{
        self.chunk_data.as_slice()
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.chunk_data.clone())
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let res = self.length.to_be_bytes();
        res.as_slice().iter().clone()
            .chain(self.chunk_type.bytes().iter().clone())
            .chain(self.chunk_data.as_slice().iter().clone())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
        // self.chunk_data.clone()
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        let mut reader = BufReader::new(value);

        let mut buffer_32: [u8; 4] = [0; 4];
        
        if let Err(_) = reader.read_exact(&mut buffer_32) {
            return Err(ChunkError::LengthByteRead);
        }


        let length:u32 = u32::from_be_bytes(buffer_32);

        if let Err(_) = reader.read_exact(&mut buffer_32) {
            return Err(ChunkError::ChunkTypeByteRead);
        }

        let chunk_type = ChunkType::try_from(buffer_32);

        if let Ok(chunk_type) = chunk_type {

            let mut chunk_data:Vec<u8> = vec!(0; length as usize);
            if let Err(_) = reader.read_exact(&mut chunk_data) {

                return Err(ChunkError::DataByteRead);
            }

            let actual_crc: u32 = CRC.checksum(&value[4..(8+length as usize)]);

            if let Err(_) = reader.read_exact(&mut buffer_32) {
                return Err(ChunkError::CrcByteRead);
            }
            let  expected_crc: u32 = u32::from_be_bytes(buffer_32);

            if actual_crc != expected_crc {
                return Err(ChunkError::CrcMismatch);
            }

            return Ok(Chunk {
                length,
                chunk_type,
                chunk_data,
                crc: actual_crc
            });
        }
        else  {
            return Err(ChunkError::ChunkTypeError(chunk_type.unwrap_err()));
        }

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
