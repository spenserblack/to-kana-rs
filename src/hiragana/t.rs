use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{
    small::{self, small_y},
    unexpected_char_error, unexpected_end_of_string,
};

pub fn t(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('た'),
        Some((_, 'i')) => hiragana.push('ち'),
        Some((_, 'u')) => hiragana.push('つ'),
        Some((_, 'e')) => hiragana.push('て'),
        Some((_, 'o')) => hiragana.push('と'),
        Some((_, 'y')) => {
            hiragana.push('ち');
            small_y(hiragana, characters)?;
        }
        Some((_, 'h')) => {
            hiragana.push('て');
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
        Some((_, 's')) => {
            hiragana.push('つ');
            match characters.next() {
                Some((_, 'u')) => {}
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((_, 't')) => {
            hiragana.push('っ');
            t(hiragana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
