use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small::small_y, unexpected_char_error, unexpected_end_of_string};

pub fn p(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('ぱ'),
        Some((_, 'i')) => hiragana.push('ぴ'),
        Some((_, 'u')) => hiragana.push('ぷ'),
        Some((_, 'e')) => hiragana.push('ぺ'),
        Some((_, 'o')) => hiragana.push('ぽ'),
        Some((_, 'y')) => {
            hiragana.push('ぴ');
            small_y(hiragana, characters)?;
        }
        Some((_, 'p')) => {
            hiragana.push('っ');
            p(hiragana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
