use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small::small_y, unexpected_char_error, unexpected_end_of_string};

pub fn b(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('バ'),
        Some((_, 'i')) => katakana.push('ビ'),
        Some((_, 'u')) => katakana.push('ブ'),
        Some((_, 'e')) => katakana.push('ベ'),
        Some((_, 'o')) => katakana.push('ボ'),
        Some((_, 'y')) => {
            katakana.push('ビ');
            small_y(katakana, characters)?;
        }
        Some((_, 'b')) => {
            katakana.push('ッ');
            b(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
