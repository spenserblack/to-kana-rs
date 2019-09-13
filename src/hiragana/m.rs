use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small::small_y, unexpected_char_error, unexpected_end_of_string};

pub fn m(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('ま'),
        Some((_, 'i')) => hiragana.push('み'),
        Some((_, 'u')) => hiragana.push('む'),
        Some((_, 'e')) => hiragana.push('め'),
        Some((_, 'o')) => hiragana.push('も'),
        Some((_, 'y')) => {
            hiragana.push('み');
            small_y(hiragana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
