use std::iter::{
    Enumerate,
    Iterator,
};
use std::str::Chars;

use crate::Error;

pub fn small(hiragana: &mut String, characters: &mut Enumerate<Chars>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('ぁ'),
        Some((_, 'i')) => hiragana.push('ぃ'),
        Some((_, 'u')) => hiragana.push('ぅ'),
        Some((_, 'e')) => hiragana.push('ぇ'),
        Some((_, 'o')) => hiragana.push('ぉ'),
        Some((i, c)) => return Err(format!("character at {} not allowed: {}", i, c)),
        None => return Err(String::from("Unexpected end of string")),
    }
    Ok(())
}
