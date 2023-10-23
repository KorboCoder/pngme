use std::{str::FromStr, fmt::Display};

#[derive(Debug, Eq, PartialEq)]
struct ChunkType(u8,u8,u8,u8);


impl ChunkType {

    fn bytes(&self) -> [u8; 4] {
        return [
            self.0,
            self.1,
            self.2,
            self.3
        ]
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_critical(&self) -> bool {
        (self.0 & 0b100000 ) ==  0
    }

    fn is_public(&self) -> bool {
        (self.1 & 0b100000 ) ==  0
    }

    fn is_reserved_bit_valid(&self) -> bool {
        (self.2 & 0b100000 ) ==  0
    }

    fn is_safe_to_copy(&self) -> bool {
        (self.3 & 0b100000 ) !=  0
    }

    fn to_string(&self) -> String {
        core::str::from_utf8(&(self.bytes())).unwrap().to_string()
    }

    fn is_valid_byte(val: &u8) -> bool {
        match val {
            65..=90 | 97..=122 => true,
            _ => false
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType{
    type Error = ();

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let res = ChunkType(value[0], value[1], value[2], value[3]);
        if res.is_valid() {
            Ok(res)

        }
        else{
            Err(())
        }
    }
}

impl FromStr for ChunkType{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let byte_str = s.as_bytes();

        if byte_str.len() != 4 || !byte_str.iter().all(is_valid_byte) {
            Err(())
        }
        else {
            Ok(ChunkType(byte_str[0],byte_str[1], byte_str[2], byte_str[3]))
        }
    }
}

impl Display for ChunkType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({},{},{},{})", self.0, self.1, self.2, self.3) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
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
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }

}
