use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small, unexpected_char_error, unexpected_end_of_string};

pub fn j(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    katakana.push('ジ');
    match characters.next() {
        Some((_, 'a')) => katakana.push(small::YA),
        Some((_, 'i')) => {}
        Some((_, 'u')) => katakana.push(small::YU),
        Some((_, 'e')) => katakana.push(small::E),
        Some((_, 'o')) => katakana.push(small::YO),
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
