use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small::small_y, unexpected_char_error, unexpected_end_of_string};

pub fn p(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('パ'),
        Some((_, 'i')) => katakana.push('ピ'),
        Some((_, 'u')) => katakana.push('プ'),
        Some((_, 'e')) => katakana.push('ペ'),
        Some((_, 'o')) => katakana.push('ポ'),
        Some((_, 'y')) => {
            katakana.push('ピ');
            small_y(katakana, characters)?;
        }
        Some((_, 'p')) => {
            katakana.push('ッ');
            p(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
