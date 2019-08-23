# Changelog

## [Unreleased](https://github.com/spenserblack/to-kana-rs/compare/v0.4.0...master)

## [0.4.0] 2019/08/23 15:02
### Added
- "x_" notation for little kana (e.g. `"xya" => "ゃ"`)
- `SmallKana` trait to convert string types to small kana
- `HalfWidth` trait to convert full-width Katakana to half-width
- Options for small or half-width kana to command-line utility

### Fixed
- Lack of support for "ヴ" and "ゔ"

## [0.3.0] 2019/08/16 11:12
### Added
- "、"', "。", "！", and "？" to recognized patterns
- `'` as a separator between n and a vowel to let them be read as 2 characters instead of 1
- Trait for converting a type to Hiragana and Katakana, implemented on `&str` and `String`
- Executable binary version

### Changed
- Improve failed pattern match error message

### Fixed
- Dropped characters when Katakana conversion would contains "ッ"
- Missing syllables for "cha", "chu", "che", and "cho"

## [0.2.0] 2019/08/13 11:41
### Added
- Diacritics
- Digraphs

### Changed
- `hira` and `kata` to accept full words, not just syllables

## 0.1.0 2019/08/13 14:56
### Added
- Basic Hiragana and Katakana (no diagraphs or diacritics)

[0.4.0]: https://github.com/spenserblack/to-kana-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/spenserblack/to-kana-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/spenserblack/to-kana-rs/compare/v0.1.0...v0.2.0
