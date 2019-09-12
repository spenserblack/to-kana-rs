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

pub fn n(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.peek() {
        Some((_, 'a')) => katakana.push('な'),
        Some((_, 'i')) => katakana.push('に'),
        Some((_, 'u')) => katakana.push('ぬ'),
        Some((_, 'e')) => katakana.push('ね'),
        Some((_, 'o')) => katakana.push('の'),
        Some((_, 'y')) => {
            katakana.push('に');
            characters.next();
            small_y(katakana, characters)?;
            return Ok(());
        }
        Some((_, _)) | None => {
            katakana.push('ん');
            return Ok(());
        }
    }
    characters.next();
    Ok(())
}
