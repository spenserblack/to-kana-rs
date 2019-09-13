use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use super::{unexpected_char_error, unexpected_end_of_string};

use crate::Error;

pub const A: char = 'ァ';
pub const I: char = 'ィ';
pub const U: char = 'ゥ';
pub const E: char = 'ェ';
pub const O: char = 'ォ';
pub const YA: char = 'ャ';
pub const YU: char = 'ュ';
pub const YO: char = 'ョ';
pub const TSU: char = 'ッ';

pub fn small(
    katakana: &mut String,
    characters: &mut Peekable<Enumerate<Chars>>,
) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push(A),
        Some((_, 'i')) => katakana.push(I),
        Some((_, 'u')) => katakana.push(U),
        Some((_, 'e')) => katakana.push(E),
        Some((_, 'o')) => katakana.push(O),
        Some((_, 'y')) => small_y(katakana, characters)?,
        Some((_, 't')) => match characters.next() {
            Some((_, 'u')) => katakana.push(TSU),
            Some((_, 's')) => match characters.next() {
                Some((_, 'u')) => katakana.push(TSU),
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            },
            Some((i, c)) => return Err(unexpected_char_error(i, c)),
            None => return Err(unexpected_end_of_string()),
        },
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}

pub fn small_y(
    katakana: &mut String,
    characters: &mut Peekable<Enumerate<Chars>>,
) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push(YA),
        Some((_, 'u')) => katakana.push(YU),
        Some((_, 'e')) => katakana.push(E),
        Some((_, 'o')) => katakana.push(YO),
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
