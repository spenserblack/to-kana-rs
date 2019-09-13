use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{small, unexpected_char_error, unexpected_end_of_string};

pub fn f(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    if let Some((_, 'f')) = characters.peek() {
        characters.next();
        hiragana.push('っ');
        return f(hiragana, characters);
    }
    hiragana.push('ふ');
    match characters.next() {
        Some((_, 'a')) => hiragana.push(small::A),
        Some((_, 'i')) => hiragana.push(small::I),
        Some((_, 'u')) => {}
        Some((_, 'e')) => hiragana.push(small::E),
        Some((_, 'o')) => hiragana.push(small::O),
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
