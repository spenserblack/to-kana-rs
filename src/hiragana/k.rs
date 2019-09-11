use std::iter::{
    Enumerate,
    Iterator,
};
use std::str::Chars;

use crate::Error;

pub fn k(hiragana: &mut String, characters: &mut Enumerate<Chars>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('か'),
        Some((_, 'i')) => hiragana.push('き'),
        Some((_, 'u')) => hiragana.push('く'),
        Some((_, 'e')) => hiragana.push('け'),
        Some((_, 'o')) => hiragana.push('こ'),
        Some((i, c)) => return Err(format!("character at {} not allowed: {}", i, c)),
        None => return Err(String::from("'k' must be followed by a vowel")),
    }
    Ok(())
}
