use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{
    small::{self, small_y},
    unexpected_char_error, unexpected_end_of_string,
};

pub fn d(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('だ'),
        Some((_, 'i')) => hiragana.push('ぢ'),
        Some((_, 'u')) => hiragana.push('づ'),
        Some((_, 'e')) => hiragana.push('で'),
        Some((_, 'o')) => hiragana.push('ど'),
        Some((_, 'y')) => {
            hiragana.push('ぢ');
            small_y(hiragana, characters)?;
        }
        Some((_, 'h')) => {
            hiragana.push('で');
            match characters.next() {
                Some((_, 'a')) => hiragana.push(small::YA),
                Some((_, 'u')) => hiragana.push(small::YU),
                Some((_, 'o')) => hiragana.push(small::YO),
                Some((_, 'i')) => hiragana.push(small::I),
                Some((_, 'e')) => hiragana.push(small::E),
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
