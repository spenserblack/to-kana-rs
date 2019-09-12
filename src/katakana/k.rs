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

pub fn k(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('か'),
        Some((_, 'i')) => katakana.push('き'),
        Some((_, 'u')) => katakana.push('く'),
        Some((_, 'e')) => katakana.push('け'),
        Some((_, 'o')) => katakana.push('こ'),
        Some((_, 'y')) => {
            katakana.push('き');
            small_y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
