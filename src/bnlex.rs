
// using
use crate::bncore::USeq;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token
{
    Add     (USeq),
    Sub     (USeq),
    Right   (USeq),
    Left    (USeq),
    Loop,
    Back,
    Out,
    In
}

const ADD: u8     = 43; // +
const SUB: u8     = 45; // -
const RIGHT: u8   = 62; // >
const LEFT: u8    = 60; // <
const LOOP: u8    = 91; // [
const BACK: u8    = 93; // ]
const OUT: u8     = 46; // .
const IN: u8      = 44; // ,

pub fn tokenize_file_from_path<T: AsRef<Path>>(path: T) -> Result<Vec<Token>, Box<dyn Error>>
{
    let mut tokens: Vec<Token> = Vec::new();

    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file).bytes().peekable();

    while let Some(byte) = reader.next() {
        let value: u8  = byte?;
        
        match value {
            ADD => tokens.push(Token::Add(get_amount_inseq_consume(value, &mut reader)?)),
            SUB => tokens.push(Token::Sub(get_amount_inseq_consume(value, &mut reader)?)),
            RIGHT => tokens.push(Token::Right(get_amount_inseq_consume(value, &mut reader)?)),
            LEFT => tokens.push(Token::Left(get_amount_inseq_consume(value, &mut reader)?)),
            LOOP => tokens.push(Token::Loop),
            BACK => tokens.push(Token::Back),
            OUT => tokens.push(Token::Out),
            IN => tokens.push(Token::In),
            _ => continue
        }
    }

    return Ok(tokens);
}

fn get_amount_inseq_consume<T>(value: u8, reader: &mut Peekable<T>) -> Result<USeq, Box<dyn Error>>
where T: Iterator<Item = io::Result<u8>>
{
    let mut count: USeq = 1;

    while let Some(Ok(byte)) = reader.peek() {
        if *byte == value {
            if count >= USeq::MAX {
                return Err(format!(
                    "Maximum allowed of identical tokens in sequence has been reached\nmax: {}\n current: {} {} tokens", 
                    USeq::MAX, 
                    count, 
                    value as char)
                    .into());
            }
            count += 1;
            reader.next();
        } else {
            break;
        }
    }

    return Ok(count);
}




/////////////////////////////////////////////////////////




#[cfg(test)]
mod lexer_tests 
{
    use super::*;
    use crate::hello_world_tokens;
    use crate::bncore::DEFAULT_BF_PATH;

    #[test]
    fn print()
    {
        let file = DEFAULT_BF_PATH;
        let tokens = tokenize_file_from_path(file).expect("Failed to tokenize the file");

        assert!(!tokens.is_empty(), "[Print Failed]\nToken vector should not be empty");

        println!("\n[Print]");
        for token in &tokens {
            print!("{:?} ", token);
        }
        println!();
    }

    #[test]
    fn compare_expected()
    {
        let file = DEFAULT_BF_PATH;
        let expected_tokens: Vec<Token> = hello_world_tokens!();

        let tokens = tokenize_file_from_path(file).expect("Failed to tokenize the file");

        assert!(
            tokens.len() == expected_tokens.len(),
            "[Compare Length Failed]\nExpected size: {}\nActual size:   {}", 
            expected_tokens.len(), 
            tokens.len()
        );

        for (i, token) in tokens.iter().enumerate() {
            assert!(
                token == &expected_tokens[i],
                "[Compare Token Failed]\nToken mismatch at index {}\nExpected token: {:?}\nActual token:   {:?}", 
                i,
                &expected_tokens[i],
                token
            );
        }
    }
}