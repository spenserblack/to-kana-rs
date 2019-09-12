use std::iter::{
    Enumerate,
    Iterator,
};
use std::str::Chars;

use super::{
    unexpected_char_error,
    unexpected_end_of_string,
};

use crate::Error;

pub const A: char = 'ぁ';
pub const I: char = 'ぃ';
pub const U: char = 'ぅ';
pub const E: char = 'ぇ';
pub const O: char = 'ぉ';
pub const YA: char = 'ゃ';
pub const YU: char = 'ゅ';
pub const YO: char = 'ょ';
pub const TSU: char = 'っ';


pub fn small(hiragana: &mut String, characters: &mut Enumerate<Chars>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push(A),
        Some((_, 'i')) => hiragana.push(I),
        Some((_, 'u')) => hiragana.push(U),
        Some((_, 'e')) => hiragana.push(E),
        Some((_, 'o')) => hiragana.push(O),
        Some((_, 'y')) => small_y(hiragana, characters)?,
        Some((_, 't')) => {
            match characters.next() {
                Some((_, 'u')) => hiragana.push(TSU),
                Some((_, 's')) => {
                    match characters.next() {
                        Some((_, 'u')) => hiragana.push(TSU),
                        Some((i, c)) => return Err(unexpected_char_error(i, c)),
                        None => return Err(unexpected_end_of_string()),
                    }
                }
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}

pub fn small_y(hiragana: &mut String, characters: &mut Enumerate<Chars>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push(YA),
        Some((_, 'u')) => hiragana.push(YU),
        Some((_, 'o')) => hiragana.push(YO),
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
