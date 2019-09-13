use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small::small_y, unexpected_char_error, unexpected_end_of_string};

pub fn m(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('マ'),
        Some((_, 'i')) => katakana.push('ミ'),
        Some((_, 'u')) => katakana.push('ム'),
        Some((_, 'e')) => katakana.push('メ'),
        Some((_, 'o')) => katakana.push('モ'),
        Some((_, 'y')) => {
            katakana.push('ミ');
            small_y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
