use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{
    small::{self, small_y},
    unexpected_char_error, unexpected_end_of_string,
};

pub fn c(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'y')) => {
            katakana.push('チ');
            small_y(katakana, characters)?;
        }
        Some((_, 'h')) => {
            katakana.push('チ');
            match characters.next() {
                Some((_, 'a')) => katakana.push(small::YA),
                Some((_, 'u')) => katakana.push(small::YU),
                Some((_, 'o')) => katakana.push(small::YO),
                Some((_, 'e')) => katakana.push(small::E),
                Some((_, 'i')) => {}
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((_, 'c')) => {
            katakana.push('ッ');
            c(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
