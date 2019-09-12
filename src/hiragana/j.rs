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
    small,
};

pub fn j(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    hiragana.push('じ');
    match characters.next() {
        Some((_, 'a')) => hiragana.push(small::YA),
        Some((_, 'i')) => {},
        Some((_, 'u')) => hiragana.push(small::YU),
        Some((_, 'e')) => hiragana.push(small::E),
        Some((_, 'o')) => hiragana.push(small::YO),
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
