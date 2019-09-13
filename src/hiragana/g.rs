use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small::small_y, unexpected_char_error, unexpected_end_of_string};

pub fn g(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('が'),
        Some((_, 'i')) => hiragana.push('ぎ'),
        Some((_, 'u')) => hiragana.push('ぐ'),
        Some((_, 'e')) => hiragana.push('げ'),
        Some((_, 'o')) => hiragana.push('ご'),
        Some((_, 'y')) => {
            hiragana.push('ぎ');
            small_y(hiragana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
