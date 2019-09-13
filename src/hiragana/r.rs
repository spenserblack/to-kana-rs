use std::iter::{
    Enumerate,
    Iterator,
    Peekable,
};
use std::str::Chars;

use crate::Error;

use super::{
    unexpected_char_error,
    unexpected_end_of_string,
    small::small_y,
};

pub fn r(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('ら'),
        Some((_, 'i')) => hiragana.push('り'),
        Some((_, 'u')) => hiragana.push('る'),
        Some((_, 'e')) => hiragana.push('れ'),
        Some((_, 'o')) => hiragana.push('ろ'),
        Some((_, 'y')) => {
            hiragana.push('り');
            small_y(hiragana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
