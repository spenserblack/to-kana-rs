# Changelog

## [Unreleased](https://github.com/spenserblack/to-kana-rs/compare/v0.2.0...master)
### Added
- "、"', "。", "！", and "？" to recognized patterns
- `'` as a separator between n and a vowel to let them be read as 2 characters instead of 1
- Trait for converting a type to Hiragana and Katakana, implemented on `&str` and `String`

### Changed
- Improve failed pattern match error message

## [0.2.0] 2019/08/13 11:41
### Added
- Diacritics
- Digraphs

### Changed
- `hira` and `kata` to accept full words, not just syllables

## 0.1.0 2019/08/13 14:56
### Added
- Basic Hiragana and Katakana (no diagraphs or diacritics)

[0.2.0]: https://github.com/spenserblack/to-kana-rs/compare/v0.1.0...v0.2.0
