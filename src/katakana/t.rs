use std::iter::{
    Enumerate,
    Iterator,
    Peekable,
};
use std::str::Chars;

use crate::Error;

use super::{
    unexpected_char_error,
    unexpected_end_of_string,
    small::{self, small_y},
};

pub fn t(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('た'),
        Some((_, 'i')) => katakana.push('ち'),
        Some((_, 'u')) => katakana.push('つ'),
        Some((_, 'e')) => katakana.push('て'),
        Some((_, 'o')) => katakana.push('と'),
        Some((_, 'y')) => {
            katakana.push('ち');
            small_y(katakana, characters)?;
        }
        Some((_, 'h')) => {
            katakana.push('て');
            match characters.next() {
                Some((_, 'a')) => katakana.push(small::YA),
                Some((_, 'u')) => katakana.push(small::YU),
                Some((_, 'o')) => katakana.push(small::YO),
                Some((_, 'i')) => katakana.push(small::I),
                Some((_, 'e')) => katakana.push(small::E),
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((_, 's')) => {
            katakana.push('つ');
            match characters.next() {
                Some((_, 'u')) => {},
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
