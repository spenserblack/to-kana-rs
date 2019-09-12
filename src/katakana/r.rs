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

pub fn r(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ら'),
        Some((_, 'i')) => katakana.push('り'),
        Some((_, 'u')) => katakana.push('る'),
        Some((_, 'e')) => katakana.push('れ'),
        Some((_, 'o')) => katakana.push('ろ'),
        Some((_, 'y')) => {
            katakana.push('り');
            small_y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
