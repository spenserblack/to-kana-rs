use std::iter::{
    Enumerate,
    Iterator,
    Peekable,
};
use std::str::Chars;

use crate::Error;

use super::{
    small::small_y,
};

pub fn n(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.peek() {
        Some((_, 'a')) => katakana.push('ナ'),
        Some((_, 'i')) => katakana.push('ニ'),
        Some((_, 'u')) => katakana.push('ヌ'),
        Some((_, 'e')) => katakana.push('ネ'),
        Some((_, 'o')) => katakana.push('ノ'),
        Some((_, 'y')) => {
            katakana.push('ニ');
            characters.next();
            small_y(katakana, characters)?;
            return Ok(());
        }
        Some((_, _)) | None => {
            katakana.push('ン');
            return Ok(());
        }
    }
    characters.next();
    Ok(())
}
