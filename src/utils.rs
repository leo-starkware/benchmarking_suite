use anyhow::Result;
use regex::Regex;
use starknet::core::types::{BlockId, BlockTag, FieldElement, FromStrError};
use url::Url;

pub fn parse_block_id(id: &str) -> Result<BlockId> {
    let regex_block_number = Regex::new("^[0-9]{1,}$").unwrap();

    if id == "latest" {
        Ok(BlockId::Tag(BlockTag::Latest))
    } else if id == "pending" {
        Ok(BlockId::Tag(BlockTag::Pending))
    } else if regex_block_number.is_match(id) {
        Ok(BlockId::Number(id.parse::<u64>()?))
    } else {
        Ok(BlockId::Hash(FieldElement::from_hex_be(id)?))
    }
}

pub fn url_checker(url_vec: &Vec<String>) {
    for url in url_vec.iter() {
        let url_check = Url::parse(url.as_str());
            url_check.unwrap();
    }
}


#[derive(Debug)]
pub enum HashStrError {
    InvalidHashLength,
    FromStrError(FromStrError)
}

pub fn hash_hex_to_fe(hash: &str) -> Result<FieldElement, HashStrError>{
    let value = hash.trim_start_matches("0x");
    let hex_chars_len = value.len();
    let expected_hex_length = 64;

    if hex_chars_len != expected_hex_length {
        Err(HashStrError::InvalidHashLength)
    } else {
        match FieldElement::from_hex_be(hash) {
            Ok(val) => Ok(val),
            Err(FromStrError::InvalidCharacter) => Err(HashStrError::FromStrError(FromStrError::InvalidCharacter)),
            Err(FromStrError::OutOfRange) => Err(HashStrError::FromStrError(FromStrError::OutOfRange)),

        }
    }
}
