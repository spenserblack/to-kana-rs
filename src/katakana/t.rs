use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{
    small::{self, small_y},
    unexpected_char_error, unexpected_end_of_string,
};

pub fn t(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('タ'),
        Some((_, 'i')) => katakana.push('チ'),
        Some((_, 'u')) => katakana.push('ツ'),
        Some((_, 'e')) => katakana.push('テ'),
        Some((_, 'o')) => katakana.push('ト'),
        Some((_, 'y')) => {
            katakana.push('チ');
            small_y(katakana, characters)?;
        }
        Some((_, 'h')) => {
            katakana.push('テ');
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
            katakana.push('ツ');
            match characters.next() {
                Some((_, 'u')) => {}
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((_, 't')) => {
            katakana.push('ッ');
            t(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
