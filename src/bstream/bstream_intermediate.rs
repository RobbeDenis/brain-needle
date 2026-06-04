use crate::bnintermediate::*;
use crate::bnerror::BNError;
use crate::bstream::ByteStreamBuilder;
use std::io::Read;

pub fn generate_bytestream_intermediate_representation<R: Read>(reader: R) -> Result<Vec<u8>, BNError> {
    let mut bytes = reader.bytes().peekable();
    let mut builder = ByteStreamBuilder::new();

    while let Some(byte) = bytes.next() {
        let token = BNTokenMatcher::match_token(byte?);
        builder.enter_token(token)?;
    }
    
    return Ok(builder.finalize()?);
}