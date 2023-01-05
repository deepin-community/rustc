// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `Emoji_Modifier` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Emoji_Modifier`](https://www.unicode.org/reports/tr51/#Emoji_Properties).
    ///
    /// The value is `true` for characters that have emoji presentation by default.
    pub struct EmojiModifier(bool) {
        abbr => "Emoji_Modifier";
        long => "Emoji_Modifier";
        human => "Emoji Modifier";

        data_table_path => "../tables/emoji_modifier.rsv";
    }

    /// The value is `true` for characters that have emoji presentation by default.
    pub fn is_emoji_modifier(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_emoji_modifier;

        // ASCII
        assert_eq!(is_emoji_modifier('\u{0000}'), false);
        assert_eq!(is_emoji_modifier('\u{0021}'), false);

        assert_eq!(is_emoji_modifier('\u{0027}'), false);
        assert_eq!(is_emoji_modifier('\u{0028}'), false);
        assert_eq!(is_emoji_modifier('\u{0029}'), false);
        assert_eq!(is_emoji_modifier('\u{002a}'), false);

        assert_eq!(is_emoji_modifier('\u{003b}'), false);
        assert_eq!(is_emoji_modifier('\u{003c}'), false);
        assert_eq!(is_emoji_modifier('\u{003d}'), false);

        assert_eq!(is_emoji_modifier('\u{007a}'), false);
        assert_eq!(is_emoji_modifier('\u{007b}'), false);
        assert_eq!(is_emoji_modifier('\u{007c}'), false);
        assert_eq!(is_emoji_modifier('\u{007d}'), false);
        assert_eq!(is_emoji_modifier('\u{007e}'), false);

        // Other BMP
        assert_eq!(is_emoji_modifier('\u{061b}'), false);
        assert_eq!(is_emoji_modifier('\u{061c}'), false);
        assert_eq!(is_emoji_modifier('\u{061d}'), false);

        assert_eq!(is_emoji_modifier('\u{200d}'), false);
        assert_eq!(is_emoji_modifier('\u{200e}'), false);
        assert_eq!(is_emoji_modifier('\u{200f}'), false);
        assert_eq!(is_emoji_modifier('\u{2010}'), false);

        assert_eq!(is_emoji_modifier('\u{2029}'), false);
        assert_eq!(is_emoji_modifier('\u{202a}'), false);
        assert_eq!(is_emoji_modifier('\u{202e}'), false);
        assert_eq!(is_emoji_modifier('\u{202f}'), false);

        // Other Planes
        assert_eq!(is_emoji_modifier('\u{10000}'), false);
        assert_eq!(is_emoji_modifier('\u{10001}'), false);

        assert_eq!(is_emoji_modifier('\u{1f1e5}'), false);
        assert_eq!(is_emoji_modifier('\u{1f1e6}'), false);
        assert_eq!(is_emoji_modifier('\u{1f1ff}'), false);
        assert_eq!(is_emoji_modifier('\u{1f200}'), false);

        assert_eq!(is_emoji_modifier('\u{20000}'), false);
        assert_eq!(is_emoji_modifier('\u{30000}'), false);
        assert_eq!(is_emoji_modifier('\u{40000}'), false);
        assert_eq!(is_emoji_modifier('\u{50000}'), false);
        assert_eq!(is_emoji_modifier('\u{60000}'), false);
        assert_eq!(is_emoji_modifier('\u{70000}'), false);
        assert_eq!(is_emoji_modifier('\u{80000}'), false);
        assert_eq!(is_emoji_modifier('\u{90000}'), false);
        assert_eq!(is_emoji_modifier('\u{a0000}'), false);
        assert_eq!(is_emoji_modifier('\u{b0000}'), false);
        assert_eq!(is_emoji_modifier('\u{c0000}'), false);
        assert_eq!(is_emoji_modifier('\u{d0000}'), false);
        assert_eq!(is_emoji_modifier('\u{e0000}'), false);

        assert_eq!(is_emoji_modifier('\u{efffe}'), false);
        assert_eq!(is_emoji_modifier('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_emoji_modifier('\u{f0000}'), false);
        assert_eq!(is_emoji_modifier('\u{f0001}'), false);
        assert_eq!(is_emoji_modifier('\u{ffffe}'), false);
        assert_eq!(is_emoji_modifier('\u{fffff}'), false);
        assert_eq!(is_emoji_modifier('\u{100000}'), false);
        assert_eq!(is_emoji_modifier('\u{100001}'), false);
        assert_eq!(is_emoji_modifier('\u{10fffe}'), false);
        assert_eq!(is_emoji_modifier('\u{10ffff}'), false);
    }
}
