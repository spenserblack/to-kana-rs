use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small::small_y, unexpected_char_error, unexpected_end_of_string};

pub fn z(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ザ'),
        Some((_, 'i')) => katakana.push('ジ'),
        Some((_, 'u')) => katakana.push('ズ'),
        Some((_, 'e')) => katakana.push('ゼ'),
        Some((_, 'o')) => katakana.push('ゾ'),
        Some((_, 'y')) => {
            katakana.push('ジ');
            small_y(katakana, characters)?;
        }
        Some((_, 'z')) => {
            katakana.push('ッ');
            z(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
