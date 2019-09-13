use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small::small_y, unexpected_char_error, unexpected_end_of_string};

pub fn k(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('か'),
        Some((_, 'i')) => hiragana.push('き'),
        Some((_, 'u')) => hiragana.push('く'),
        Some((_, 'e')) => hiragana.push('け'),
        Some((_, 'o')) => hiragana.push('こ'),
        Some((_, 'y')) => {
            hiragana.push('き');
            small_y(hiragana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
