use kwtsms::clean_message;

#[test]
fn test_plain_text_unchanged() {
    assert_eq!(clean_message("Hello World"), "Hello World");
}

#[test]
fn test_arabic_indic_digits_converted() {
    // ١٢٣ -> 123
    assert_eq!(clean_message("Order \u{0661}\u{0662}\u{0663}"), "Order 123");
}

#[test]
fn test_persian_digits_converted() {
    // ۴۵۶ -> 456
    assert_eq!(clean_message("Code \u{06F4}\u{06F5}\u{06F6}"), "Code 456");
}

#[test]
fn test_emoji_removed() {
    assert_eq!(clean_message("Hello \u{1F389}"), "Hello ");
}

#[test]
fn test_multiple_emojis_removed() {
    assert_eq!(clean_message("\u{1F389}\u{1F38A}\u{1F680}"), "");
}

#[test]
fn test_emoji_only_message() {
    assert_eq!(clean_message("\u{1F600}\u{1F601}\u{1F602}"), "");
}

#[test]
fn test_emoticons_range() {
    assert_eq!(clean_message("Hi \u{1F600} \u{1F64F}"), "Hi  ");
}

#[test]
fn test_transport_range() {
    assert_eq!(clean_message("Go \u{1F680}"), "Go ");
}

#[test]
fn test_misc_symbols() {
    assert_eq!(clean_message("Sun \u{2600}"), "Sun ");
}

#[test]
fn test_dingbats() {
    assert_eq!(clean_message("Cut \u{2702}"), "Cut ");
}

#[test]
fn test_variation_selectors() {
    assert_eq!(clean_message("Star\u{FE0F}"), "Star");
}

#[test]
fn test_keycap() {
    assert_eq!(clean_message("1\u{20E3}"), "1");
}

#[test]
fn test_regional_indicators() {
    assert_eq!(clean_message("\u{1F1FA}\u{1F1F8}"), "");
}

#[test]
fn test_tags_block() {
    assert_eq!(clean_message("Text\u{E0001}"), "Text");
}

#[test]
fn test_html_stripped() {
    assert_eq!(clean_message("Hello<b>World</b>"), "HelloWorld");
}

#[test]
fn test_html_with_attributes() {
    assert_eq!(clean_message("<p class=\"x\">Hello</p>"), "Hello");
}

#[test]
fn test_html_nested() {
    assert_eq!(clean_message("<div><p>Hello</p></div>"), "Hello");
}

#[test]
fn test_zero_width_space() {
    assert_eq!(clean_message("Hello\u{200B}World"), "HelloWorld");
}

#[test]
fn test_zero_width_non_joiner() {
    assert_eq!(clean_message("Hello\u{200C}World"), "HelloWorld");
}

#[test]
fn test_zero_width_joiner() {
    assert_eq!(clean_message("Hello\u{200D}World"), "HelloWorld");
}

#[test]
fn test_word_joiner() {
    assert_eq!(clean_message("Hello\u{2060}World"), "HelloWorld");
}

#[test]
fn test_soft_hyphen() {
    assert_eq!(clean_message("Hel\u{00AD}lo"), "Hello");
}

#[test]
fn test_bom() {
    assert_eq!(clean_message("\u{FEFF}Hello"), "Hello");
}

#[test]
fn test_object_replacement() {
    assert_eq!(clean_message("Hello\u{FFFC}World"), "HelloWorld");
}

#[test]
fn test_ltr_mark() {
    assert_eq!(clean_message("Hello\u{200E}World"), "HelloWorld");
}

#[test]
fn test_rtl_mark() {
    assert_eq!(clean_message("Hello\u{200F}World"), "HelloWorld");
}

#[test]
fn test_directional_formatting() {
    assert_eq!(clean_message("A\u{202A}B\u{202C}C"), "ABC");
}

#[test]
fn test_directional_isolates() {
    assert_eq!(clean_message("A\u{2066}B\u{2069}C"), "ABC");
}

#[test]
fn test_null_char() {
    assert_eq!(clean_message("Hello\0World"), "HelloWorld");
}

#[test]
fn test_c0_controls_except_newline_tab() {
    assert_eq!(clean_message("Hello\x01\x02\x03World"), "HelloWorld");
}

#[test]
fn test_del_char() {
    assert_eq!(clean_message("Hello\x7FWorld"), "HelloWorld");
}

#[test]
fn test_c1_controls() {
    assert_eq!(clean_message("Hello\u{0080}\u{009F}World"), "HelloWorld");
}

#[test]
fn test_preserve_newline() {
    assert_eq!(clean_message("Hello\nWorld"), "Hello\nWorld");
}

#[test]
fn test_preserve_tab() {
    assert_eq!(clean_message("Hello\tWorld"), "Hello\tWorld");
}

#[test]
fn test_preserve_arabic_text() {
    let arabic = "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}"; // مرحبا
    assert_eq!(clean_message(arabic), arabic);
}

#[test]
fn test_preserve_arabic_with_latin() {
    assert_eq!(
        clean_message("Hello \u{0645}\u{0631}\u{062D}\u{0628}\u{0627}"),
        "Hello \u{0645}\u{0631}\u{062D}\u{0628}\u{0627}"
    );
}

#[test]
fn test_complex_message() {
    assert_eq!(
        clean_message("OTP: \u{0661}\u{0662}\u{0663}\u{0664} \u{1F600}\u{200B}<b>Done</b>"),
        "OTP: 1234 Done"
    );
}

#[test]
fn test_mahjong_tiles() {
    assert_eq!(clean_message("Game \u{1F004}"), "Game ");
}

#[test]
fn test_playing_cards() {
    assert_eq!(clean_message("Card \u{1F0A1}"), "Card ");
}

#[test]
fn test_alchemical() {
    assert_eq!(clean_message("Symbol \u{1F700}"), "Symbol ");
}

#[test]
fn test_geometric_extended() {
    assert_eq!(clean_message("Shape \u{1F780}"), "Shape ");
}

#[test]
fn test_supplemental_arrows() {
    assert_eq!(clean_message("Arrow \u{1F800}"), "Arrow ");
}

#[test]
fn test_supplemental_symbols() {
    assert_eq!(clean_message("Sym \u{1F900}"), "Sym ");
}

#[test]
fn test_chess_symbols() {
    assert_eq!(clean_message("Chess \u{1FA00}"), "Chess ");
}

#[test]
fn test_extended_pictographs() {
    assert_eq!(clean_message("Pic \u{1FA70}"), "Pic ");
}
