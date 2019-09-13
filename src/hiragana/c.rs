use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{
    small::{self, small_y},
    unexpected_char_error, unexpected_end_of_string,
};

pub fn c(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'y')) => {
            hiragana.push('ち');
            small_y(hiragana, characters)?;
        }
        Some((_, 'h')) => {
            hiragana.push('ち');
            match characters.next() {
                Some((_, 'a')) => hiragana.push(small::YA),
                Some((_, 'u')) => hiragana.push(small::YU),
                Some((_, 'o')) => hiragana.push(small::YO),
                Some((_, 'e')) => hiragana.push(small::E),
                Some((_, 'i')) => {}
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
