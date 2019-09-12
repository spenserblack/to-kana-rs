use std::iter::{
    Enumerate,
    Iterator,
};
use std::str::Chars;

use crate::Error;

use super::small::{self, small_y};

pub fn s(hiragana: &mut String, characters: &mut Enumerate<Chars>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('さ'),
        Some((_, 'i')) => hiragana.push('し'),
        Some((_, 'u')) => hiragana.push('す'),
        Some((_, 'e')) => hiragana.push('せ'),
        Some((_, 'o')) => hiragana.push('そ'),
        Some((_, 'y')) | Some((_, 'h')) => {
            hiragana.push('し');
            match characters.next() {
                Some((_, 'a')) => hiragana.push(small::YA),
                Some((_, 'u')) => hiragana.push(small::YU),
                Some((_, 'o')) => hiragana.push(small::YO),
                Some((_, 'e')) => hiragana.push(small::E),
                Some((_, 'i')) => {},
                Some((i, c)) => return Err(format!("Unexpected char at {}: {}", i, c)),
                None => return Err(String::from("Unexpected end of string")),
            }
        }
        Some((i, c)) => return Err(format!("character at {} not allowed: {}", i, c)),
        None => return Err(String::from("'s' must be followed by a vowel")),
    }
    Ok(())
}
