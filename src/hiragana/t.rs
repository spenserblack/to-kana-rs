use std::iter::{
    Enumerate,
    Iterator,
};
use std::str::Chars;

use crate::Error;

use super::{
    unexpected_char_error,
    unexpected_end_of_string,
    small::{self, small_y},
};

pub fn t(hiragana: &mut String, characters: &mut Enumerate<Chars>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('た'),
        Some((_, 'i')) => hiragana.push('ち'),
        Some((_, 'u')) => hiragana.push('つ'),
        Some((_, 'e')) => hiragana.push('て'),
        Some((_, 'o')) => hiragana.push('と'),
        Some((_, 'y')) => {
            hiragana.push('ち');
            match characters.next() {
                Some((_, 'a')) => hiragana.push(small::YA),
                Some((_, 'u')) => hiragana.push(small::YU),
                Some((_, 'o')) => hiragana.push(small::YO),
                Some((_, 'e')) => hiragana.push(small::E),
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((_, 's')) => {
            hiragana.push('つ');
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
