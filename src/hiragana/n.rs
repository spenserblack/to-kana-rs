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

pub fn n(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.peek() {
        Some((_, 'a')) => hiragana.push('な'),
        Some((_, 'i')) => hiragana.push('に'),
        Some((_, 'u')) => hiragana.push('ぬ'),
        Some((_, 'e')) => hiragana.push('ね'),
        Some((_, 'o')) => hiragana.push('の'),
        Some((_, 'y')) => {
            hiragana.push('に');
            characters.next();
            small_y(hiragana, characters)?;
            return Ok(());
        }
        Some((_, _)) | None => {
            hiragana.push('ん');
            return Ok(());
        }
    }
    characters.next();
    Ok(())
}
