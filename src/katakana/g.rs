use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small::small_y, unexpected_char_error, unexpected_end_of_string};

pub fn g(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ガ'),
        Some((_, 'i')) => katakana.push('ギ'),
        Some((_, 'u')) => katakana.push('グ'),
        Some((_, 'e')) => katakana.push('ゲ'),
        Some((_, 'o')) => katakana.push('ゴ'),
        Some((_, 'y')) => {
            katakana.push('ギ');
            small_y(katakana, characters)?;
        }
        Some((_, 'g')) => {
            katakana.push('ッ');
            g(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
