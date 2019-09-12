use std::iter::{
    Enumerate,
    Iterator,
};
use std::str::Chars;

use crate::Error;

pub const A: char = 'ぁ';
pub const I: char = 'ぃ';
pub const U: char = 'ぅ';
pub const E: char = 'ぇ';
pub const O: char = 'ぉ';
pub const YA: char = 'ゃ';
pub const YU: char = 'ゅ';
pub const YO: char = 'ょ';


pub fn small(hiragana: &mut String, characters: &mut Enumerate<Chars>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push(A),
        Some((_, 'i')) => hiragana.push(I),
        Some((_, 'u')) => hiragana.push(U),
        Some((_, 'e')) => hiragana.push(E),
        Some((_, 'o')) => hiragana.push(O),
        Some((_, 'y')) => small_y(hiragana, characters)?,
        Some((i, c)) => return Err(format!("character at {} not allowed: {}", i, c)),
        None => return Err(String::from("Unexpected end of string")),
    }
    Ok(())
}

pub fn small_y(hiragana: &mut String, characters: &mut Enumerate<Chars>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push(YA),
        Some((_, 'u')) => hiragana.push(YU),
        Some((_, 'o')) => hiragana.push(YO),
        Some((i, c)) => return Err(format!("character at {} not allowed: {}", i, c)),
        None => return Err(String::from("Unexpected end of string")),
    }
    Ok(())
}
