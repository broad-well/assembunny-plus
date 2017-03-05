use parser;
use parser::{Token, TokenType};
use std::iter;
use std::io::Cursor;

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

// Bytecode management for Assembunny-plus
// Bytecode binary files are in '.asmbb'

// Assembunny-plus Bytecode Specification
// (known in this passage as "ASMBP Bytecode")
// An ASMBP Bytecode file contains two segments,
// first segment represents file metadata (amount of registers to allocate, etc.)
// second segment represents tokens.
// The first segment is 32 bytes long. Contents are follows (each '-' represents one byte):
//
// |----:----------------------------|
//   |                     |
// [Register count]  [Reserved for future use]
//
// The second segment consists of token representation Blobs, each 5 bytes long.
// A token representation Blob consists of the following (each '-' represents one bit):
//
// |--------:--------:--------:--------:--------|
//   ^           \_______|________|_______/
//   |                        |
//  [Type in u8]        [Data in i32]
//
// Since every line of ASMB+ starts with a KEYWORD token, the tokens provided in the ASMBP Bytecode file are split whenever a new KEYWORD token is reached while iterating.

// Converts a given ASMBP program to bytecode.
// The program (parameter of this fn) should be a Slice of Strings containing single ASMBP statements.
pub fn to_bytecode(asmbp: &Vec<&str>) -> Result<Vec<u8>, String> {
    let mut segment1: Vec<u8> = Vec::new();
    let mut segment2: Vec<u8> = Vec::new();
    let mut regs: Vec<String> = Vec::new();
    
    for line in asmbp {
        if let Some(tokens) = try!(parser::to_tokens(line, &mut regs)) {
            for token in tokens {
                segment2.append(&mut token.to_bytearray());
            }
        }
    }

    // Querying length from regs after filling segment2 because regs also gets filled in the process.
    segment1.write_u32::<BigEndian>(regs.len() as u32).unwrap();
    segment1.extend(iter::repeat(0u8).take(28 /* 32 - 4 */));
    assert_eq!(segment1.len(), 32);

    segment1.append(&mut segment2);
    Ok(segment1)
}

// Converts a given bytecode sequence (Vec<u8>) to (usize /* register count */, Vec<Vec<Token>>).
pub fn from_bytecode(bytecode: &Vec<u8>) -> Result<(usize, Vec<Vec<Token>>), String> {
    let mut seg1reader = Cursor::new(&bytecode[0..4]);
    let reg_count = try_failsafe!(seg1reader.read_u32::<BigEndian>(), "Failed to read register count in metadata".to_owned()) as usize;

    let segment2 = bytecode[32..].chunks(5);
    let mut toks: Vec<Vec<Token>> = Vec::new();

    for (index, bytoken) in segment2.enumerate() {
        let token = try_err_fallthru!(Token::from_bytearray(&bytoken),
                                      format!("Failed to convert from bytes to Token in chunk index {}: ", index));
        if token.type_ == TokenType::KEYWORD {
            toks.push(vec![token]);
        } else {
            try_opt!(toks.last_mut(),
                     "First token is not of type KEYWORD".to_owned()).push(token);
        }
    }
    Ok((reg_count, toks))
}