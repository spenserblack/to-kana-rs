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
    small::{self, small_y},
};

pub fn f(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    katakana.push('ふ');
    match characters.next() {
        Some((_, 'a')) => katakana.push(small::A),
        Some((_, 'i')) => katakana.push(small::I),
        Some((_, 'u')) => {},
        Some((_, 'e')) => katakana.push(small::E),
        Some((_, 'o')) => katakana.push(small::O),
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
