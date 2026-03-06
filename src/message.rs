/// Clean SMS message text before sending.
///
/// Processing order:
/// 1. Convert Arabic-Indic and Extended Arabic-Indic digits to Latin
/// 2. Remove emojis (all major emoji Unicode ranges)
/// 3. Remove hidden invisible characters (zero-width space, BOM, soft hyphen, etc.)
/// 4. Remove directional formatting characters
/// 5. Remove C0/C1 control characters (preserve \n and \t)
/// 6. Strip HTML tags
///
/// Arabic letters are preserved (fully supported by kwtSMS).
pub fn clean_message(text: &str) -> String {
    let mut result = String::with_capacity(text.len());

    for c in text.chars() {
        // 1. Convert Arabic-Indic digits (U+0660-U+0669) to Latin
        if ('\u{0660}'..='\u{0669}').contains(&c) {
            result.push((b'0' + (c as u32 - 0x0660) as u8) as char);
            continue;
        }
        // Convert Extended Arabic-Indic / Persian digits (U+06F0-U+06F9) to Latin
        if ('\u{06F0}'..='\u{06F9}').contains(&c) {
            result.push((b'0' + (c as u32 - 0x06F0) as u8) as char);
            continue;
        }

        // 2. Remove emojis
        if is_emoji(c) {
            continue;
        }

        // 3. Remove hidden invisible characters
        if is_hidden_char(c) {
            continue;
        }

        // 4. Remove directional formatting characters
        if is_directional_char(c) {
            continue;
        }

        // 5. Remove C0/C1 control characters (preserve \n U+000A and \t U+0009)
        if is_control_char(c) && c != '\n' && c != '\t' {
            continue;
        }

        result.push(c);
    }

    // 6. Strip HTML tags
    strip_html_tags(&result)
}

fn is_emoji(c: char) -> bool {
    let cp = c as u32;
    matches!(cp,
        0x1F000..=0x1F02F |  // Mahjong, domino tiles
        0x1F0A0..=0x1F0FF |  // Playing cards
        0x1F1E0..=0x1F1FF |  // Regional indicator symbols / flag components
        0x1F300..=0x1F5FF |  // Misc symbols and pictographs
        0x1F600..=0x1F64F |  // Emoticons
        0x1F680..=0x1F6FF |  // Transport and map
        0x1F700..=0x1F77F |  // Alchemical symbols
        0x1F780..=0x1F7FF |  // Geometric shapes extended
        0x1F800..=0x1F8FF |  // Supplemental arrows
        0x1F900..=0x1F9FF |  // Supplemental symbols and pictographs
        0x1FA00..=0x1FA6F |  // Chess symbols
        0x1FA70..=0x1FAFF |  // Symbols and pictographs extended
        0x2600..=0x26FF   |  // Misc symbols
        0x2700..=0x27BF   |  // Dingbats
        0xFE00..=0xFE0F   |  // Variation selectors
        0x20E3            |  // Combining enclosing keycap
        0xE0000..=0xE007F    // Tags block (subdivision flags)
    )
}

fn is_hidden_char(c: char) -> bool {
    matches!(
        c,
        '\u{200B}' |  // Zero-width space
        '\u{200C}' |  // Zero-width non-joiner
        '\u{200D}' |  // Zero-width joiner
        '\u{2060}' |  // Word joiner
        '\u{00AD}' |  // Soft hyphen
        '\u{FEFF}' |  // BOM
        '\u{FFFC}' // Object replacement character
    )
}

fn is_directional_char(c: char) -> bool {
    matches!(c,
        '\u{200E}' |         // Left-to-right mark
        '\u{200F}' |         // Right-to-left mark
        '\u{202A}'..='\u{202E}' |  // LRE, RLE, PDF, LRO, RLO
        '\u{2066}'..='\u{2069}'    // LRI, RLI, FSI, PDI
    )
}

fn is_control_char(c: char) -> bool {
    let cp = c as u32;
    matches!(cp,
        0x0000..=0x001F |  // C0 controls
        0x007F          |  // DEL
        0x0080..=0x009F    // C1 controls
    )
}

fn strip_html_tags(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut in_tag = false;
    for c in text.chars() {
        if c == '<' {
            in_tag = true;
            continue;
        }
        if c == '>' && in_tag {
            in_tag = false;
            continue;
        }
        if !in_tag {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_plain_text() {
        assert_eq!(clean_message("Hello World"), "Hello World");
    }

    #[test]
    fn test_clean_arabic_digits() {
        assert_eq!(clean_message("Order \u{0661}\u{0662}\u{0663}"), "Order 123");
    }

    #[test]
    fn test_clean_persian_digits() {
        assert_eq!(clean_message("Code \u{06F4}\u{06F5}\u{06F6}"), "Code 456");
    }

    #[test]
    fn test_clean_emojis() {
        assert_eq!(clean_message("Hello \u{1F389}"), "Hello ");
    }

    #[test]
    fn test_clean_multiple_emojis() {
        assert_eq!(clean_message("\u{1F389}\u{1F38A}\u{1F680}"), "");
    }

    #[test]
    fn test_clean_emoji_only() {
        assert_eq!(clean_message("\u{1F600}\u{1F601}\u{1F602}"), "");
    }

    #[test]
    fn test_clean_html_tags() {
        assert_eq!(clean_message("Hello<b>World</b>"), "HelloWorld");
    }

    #[test]
    fn test_clean_html_with_attributes() {
        assert_eq!(clean_message("<p class=\"test\">Hello</p>"), "Hello");
    }

    #[test]
    fn test_clean_zero_width_space() {
        assert_eq!(clean_message("Hello\u{200B}World"), "HelloWorld");
    }

    #[test]
    fn test_clean_bom() {
        assert_eq!(clean_message("\u{FEFF}Hello"), "Hello");
    }

    #[test]
    fn test_clean_soft_hyphen() {
        assert_eq!(clean_message("Hel\u{00AD}lo"), "Hello");
    }

    #[test]
    fn test_clean_zero_width_joiner() {
        assert_eq!(clean_message("Hello\u{200D}World"), "HelloWorld");
    }

    #[test]
    fn test_clean_preserves_arabic_text() {
        assert_eq!(
            clean_message("\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}"),
            "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}"
        );
    }

    #[test]
    fn test_clean_preserves_newlines() {
        assert_eq!(clean_message("Hello\nWorld"), "Hello\nWorld");
    }

    #[test]
    fn test_clean_preserves_tabs() {
        assert_eq!(clean_message("Hello\tWorld"), "Hello\tWorld");
    }

    #[test]
    fn test_clean_removes_null() {
        assert_eq!(clean_message("Hello\0World"), "HelloWorld");
    }

    #[test]
    fn test_clean_directional_marks() {
        assert_eq!(clean_message("Hello\u{200E}World"), "HelloWorld");
        assert_eq!(clean_message("Hello\u{200F}World"), "HelloWorld");
    }

    #[test]
    fn test_clean_c1_controls() {
        assert_eq!(clean_message("Hello\u{0080}World"), "HelloWorld");
        assert_eq!(clean_message("Hello\u{009F}World"), "HelloWorld");
    }

    #[test]
    fn test_clean_variation_selectors() {
        assert_eq!(clean_message("Hello\u{FE0F}World"), "HelloWorld");
    }

    #[test]
    fn test_clean_misc_symbols() {
        // U+2600 = Black sun with rays
        assert_eq!(clean_message("Weather \u{2600}"), "Weather ");
    }

    #[test]
    fn test_clean_dingbats() {
        // U+2702 = Black scissors
        assert_eq!(clean_message("Cut \u{2702}"), "Cut ");
    }

    #[test]
    fn test_clean_keycap() {
        // U+20E3 = Combining enclosing keycap
        assert_eq!(clean_message("1\u{20E3}"), "1");
    }

    #[test]
    fn test_clean_regional_indicators() {
        // U+1F1FA U+1F1F8 = US flag
        assert_eq!(clean_message("\u{1F1FA}\u{1F1F8}"), "");
    }

    #[test]
    fn test_clean_tags_block() {
        assert_eq!(clean_message("\u{E0001}"), "");
    }

    #[test]
    fn test_clean_complex_message() {
        assert_eq!(
            clean_message(
                "Your OTP is \u{0661}\u{0662}\u{0663}\u{0664} \u{1F600}\u{200B}<b>Hello</b>"
            ),
            "Your OTP is 1234 Hello"
        );
    }
}
