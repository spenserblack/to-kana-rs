use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{unexpected_char_error, unexpected_end_of_string};

pub fn y(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ヤ'),
        Some((_, 'u')) => katakana.push('ユ'),
        Some((_, 'o')) => katakana.push('ヨ'),
        Some((_, 'y')) => {
            katakana.push('ッ');
            y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
