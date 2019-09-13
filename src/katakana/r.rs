use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small::small_y, unexpected_char_error, unexpected_end_of_string};

pub fn r(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ラ'),
        Some((_, 'i')) => katakana.push('リ'),
        Some((_, 'u')) => katakana.push('ル'),
        Some((_, 'e')) => katakana.push('レ'),
        Some((_, 'o')) => katakana.push('ロ'),
        Some((_, 'y')) => {
            katakana.push('リ');
            small_y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
