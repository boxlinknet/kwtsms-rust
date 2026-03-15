use kwtsms::{find_country_code, normalize_phone, validate_phone_format, validate_phone_input};

// ===== normalize_phone tests =====

#[test]
fn test_normalize_plain() {
    assert_eq!(normalize_phone("96598765432"), "96598765432");
}

#[test]
fn test_normalize_plus() {
    assert_eq!(normalize_phone("+96598765432"), "96598765432");
}

#[test]
fn test_normalize_double_zero() {
    assert_eq!(normalize_phone("0096598765432"), "96598765432");
}

#[test]
fn test_normalize_spaces() {
    assert_eq!(normalize_phone("965 9876 5432"), "96598765432");
}

#[test]
fn test_normalize_dashes() {
    assert_eq!(normalize_phone("965-9876-5432"), "96598765432");
}

#[test]
fn test_normalize_dots() {
    assert_eq!(normalize_phone("965.9876.5432"), "96598765432");
}

#[test]
fn test_normalize_parens() {
    assert_eq!(normalize_phone("(965) 98765432"), "96598765432");
}

#[test]
fn test_normalize_slashes() {
    assert_eq!(normalize_phone("965/9876/5432"), "96598765432");
}

#[test]
fn test_normalize_arabic_indic() {
    assert_eq!(
        normalize_phone(
            "\u{0669}\u{0666}\u{0665}\u{0669}\u{0668}\u{0667}\u{0666}\u{0665}\u{0664}\u{0663}\u{0662}"
        ),
        "96598765432"
    );
}

#[test]
fn test_normalize_persian_digits() {
    assert_eq!(
        normalize_phone(
            "\u{06F9}\u{06F6}\u{06F5}\u{06F9}\u{06F8}\u{06F7}\u{06F6}\u{06F5}\u{06F4}\u{06F3}\u{06F2}"
        ),
        "96598765432"
    );
}

#[test]
fn test_normalize_mixed_formats() {
    assert_eq!(normalize_phone("+00 965-9876-5432"), "96598765432");
}

#[test]
fn test_normalize_empty() {
    assert_eq!(normalize_phone(""), "");
}

#[test]
fn test_normalize_whitespace_only() {
    assert_eq!(normalize_phone("   "), "");
}

#[test]
fn test_normalize_no_digits() {
    assert_eq!(normalize_phone("abc"), "");
}

#[test]
fn test_normalize_leading_zeros() {
    assert_eq!(normalize_phone("0098765432"), "98765432");
}

#[test]
fn test_normalize_all_zeros() {
    assert_eq!(normalize_phone("000"), "");
}

#[test]
fn test_normalize_brackets() {
    assert_eq!(normalize_phone("[965] 98765432"), "96598765432");
}

#[test]
fn test_normalize_mixed_arabic_latin() {
    assert_eq!(normalize_phone("965\u{0669}876\u{0665}432"), "96598765432");
}

#[test]
fn test_normalize_leading_trailing_whitespace() {
    assert_eq!(normalize_phone("  96598765432  "), "96598765432");
}

// ===== Domestic trunk prefix stripping =====

#[test]
fn test_normalize_saudi_trunk_0559() {
    // 9660559876543 -> 966559876543
    assert_eq!(normalize_phone("9660559876543"), "966559876543");
}

#[test]
fn test_normalize_saudi_trunk_with_plus() {
    assert_eq!(normalize_phone("+9660559876543"), "966559876543");
}

#[test]
fn test_normalize_saudi_trunk_with_00() {
    assert_eq!(normalize_phone("009660559876543"), "966559876543");
}

#[test]
fn test_normalize_uae_trunk_050() {
    // 9710501234567 -> 971501234567
    assert_eq!(normalize_phone("9710501234567"), "971501234567");
}

#[test]
fn test_normalize_egypt_trunk_010() {
    // 20010123456789 -> 2010123456789
    assert_eq!(normalize_phone("20010123456789"), "2010123456789");
}

#[test]
fn test_normalize_no_trunk_no_change() {
    assert_eq!(normalize_phone("966559876543"), "966559876543");
}

#[test]
fn test_normalize_kuwait_no_trunk() {
    assert_eq!(normalize_phone("96598765432"), "96598765432");
}

#[test]
fn test_normalize_jordan_trunk_079() {
    // 9620791234567 -> 962791234567
    assert_eq!(normalize_phone("9620791234567"), "962791234567");
}

// ===== find_country_code tests =====

#[test]
fn test_find_cc_kuwait_3digit() {
    assert_eq!(find_country_code("96598765432"), Some("965"));
}

#[test]
fn test_find_cc_usa_1digit() {
    assert_eq!(find_country_code("12025551234"), Some("1"));
}

#[test]
fn test_find_cc_egypt_2digit() {
    assert_eq!(find_country_code("201012345678"), Some("20"));
}

#[test]
fn test_find_cc_unknown() {
    assert_eq!(find_country_code("99912345678"), None);
}

#[test]
fn test_find_cc_empty() {
    assert_eq!(find_country_code(""), None);
}

#[test]
fn test_find_cc_saudi() {
    assert_eq!(find_country_code("966559876543"), Some("966"));
}

#[test]
fn test_find_cc_uk() {
    assert_eq!(find_country_code("447911123456"), Some("44"));
}

// ===== validate_phone_format tests =====

#[test]
fn test_format_kuwait_valid() {
    let (valid, _) = validate_phone_format("96598765432");
    assert!(valid);
}

#[test]
fn test_format_kuwait_wrong_length() {
    let (valid, err) = validate_phone_format("9659876543");
    assert!(!valid);
    assert!(err.unwrap().contains("expected 8 digits"));
}

#[test]
fn test_format_kuwait_wrong_prefix() {
    let (valid, err) = validate_phone_format("96512345678");
    assert!(!valid);
    assert!(err.unwrap().contains("must start with"));
}

#[test]
fn test_format_kuwait_all_valid_prefixes() {
    for prefix in ['4', '5', '6', '9'] {
        let number = format!("965{}1234567", prefix);
        let (valid, err) = validate_phone_format(&number);
        assert!(
            valid,
            "Kuwait prefix {} should be valid, got: {:?}",
            prefix, err
        );
    }
}

#[test]
fn test_format_saudi_valid() {
    let (valid, _) = validate_phone_format("966559876543");
    assert!(valid);
}

#[test]
fn test_format_saudi_wrong_prefix() {
    let (valid, err) = validate_phone_format("966159876543");
    assert!(!valid);
    assert!(err.unwrap().contains("must start with 5"));
}

#[test]
fn test_format_uae_valid() {
    let (valid, _) = validate_phone_format("971501234567");
    assert!(valid);
}

#[test]
fn test_format_unknown_country_passes() {
    let (valid, _) = validate_phone_format("99912345678");
    assert!(valid);
}

#[test]
fn test_format_usa_valid() {
    let (valid, _) = validate_phone_format("12025551234");
    assert!(valid);
}

#[test]
fn test_format_belgium_no_prefix_check() {
    let (valid, _) = validate_phone_format("32412345678");
    assert!(valid);
}

#[test]
fn test_format_india_valid() {
    let (valid, _) = validate_phone_format("919876543210");
    assert!(valid);
}

#[test]
fn test_format_india_wrong_prefix() {
    let (valid, err) = validate_phone_format("911234567890");
    assert!(!valid);
    assert!(err.unwrap().contains("India"));
}

// ===== validate_phone_input tests =====

#[test]
fn test_validate_valid_number() {
    let (valid, err, normalized) = validate_phone_input("+96598765432");
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "96598765432");
}

#[test]
fn test_validate_empty() {
    let (valid, err, _) = validate_phone_input("");
    assert!(!valid);
    assert_eq!(err.unwrap(), "Phone number is required");
}

#[test]
fn test_validate_blank() {
    let (valid, err, _) = validate_phone_input("   ");
    assert!(!valid);
    assert_eq!(err.unwrap(), "Phone number is required");
}

#[test]
fn test_validate_email() {
    let (valid, err, _) = validate_phone_input("user@gmail.com");
    assert!(!valid);
    let err_msg = err.unwrap();
    assert!(err_msg.contains("email address"));
    assert!(err_msg.contains("user@gmail.com"));
}

#[test]
fn test_validate_no_digits() {
    let (valid, err, _) = validate_phone_input("abc");
    assert!(!valid);
    assert!(err.unwrap().contains("no digits found"));
}

#[test]
fn test_validate_too_short() {
    let (valid, err, normalized) = validate_phone_input("123456");
    assert!(!valid);
    let err_msg = err.unwrap();
    assert!(err_msg.contains("too short"));
    assert!(err_msg.contains("6 digits"));
    assert_eq!(normalized, "123456");
}

#[test]
fn test_validate_too_long() {
    let (valid, err, _) = validate_phone_input("1234567890123456");
    assert!(!valid);
    let err_msg = err.unwrap();
    assert!(err_msg.contains("too long"));
    assert!(err_msg.contains("16 digits"));
}

#[test]
fn test_validate_minimum_valid_7_digits() {
    // Use 999xxxx (no country rule) for generic E.164 test
    let (valid, err, normalized) = validate_phone_input("9991234");
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "9991234");
}

#[test]
fn test_validate_maximum_valid_15_digits() {
    // Use 999... (no country rule) for generic E.164 test
    let (valid, err, normalized) = validate_phone_input("999456789012345");
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "999456789012345");
}

#[test]
fn test_validate_exactly_16_digits() {
    let (valid, err, _) = validate_phone_input("1234567890123456");
    assert!(!valid);
    assert!(err.unwrap().contains("too long"));
}

#[test]
fn test_validate_arabic_digits() {
    let (valid, err, normalized) = validate_phone_input(
        "\u{0669}\u{0666}\u{0665}\u{0669}\u{0668}\u{0667}\u{0666}\u{0665}\u{0664}\u{0663}\u{0662}",
    );
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "96598765432");
}

#[test]
fn test_validate_double_zero_prefix() {
    let (valid, err, normalized) = validate_phone_input("0096598765432");
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "96598765432");
}

#[test]
fn test_validate_special_chars() {
    let (valid, err, _) = validate_phone_input("!@#$%^&*");
    assert!(!valid);
    assert!(err.unwrap().contains("email address"));
}

#[test]
fn test_validate_only_special_no_at() {
    let (valid, err, _) = validate_phone_input("!#$%^&*");
    assert!(!valid);
    assert!(err.unwrap().contains("no digits found"));
}

// ===== Country format validation via validate_phone_input =====

#[test]
fn test_validate_saudi_trunk_stripped_and_valid() {
    let (valid, err, normalized) = validate_phone_input("+9660559876543");
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "966559876543");
}

#[test]
fn test_validate_kuwait_wrong_prefix_rejected() {
    let (valid, err, normalized) = validate_phone_input("96512345678");
    assert!(!valid);
    assert!(err.unwrap().contains("Invalid Kuwait mobile number"));
    assert_eq!(normalized, "96512345678");
}

#[test]
fn test_validate_kuwait_wrong_length_rejected() {
    let (valid, err, _) = validate_phone_input("9659876543"); // 7 local digits
    assert!(!valid);
    assert!(err.unwrap().contains("expected 8 digits"));
}

#[test]
fn test_validate_saudi_wrong_prefix_rejected() {
    let (valid, err, _) = validate_phone_input("966159876543");
    assert!(!valid);
    assert!(err.unwrap().contains("Saudi Arabia"));
}

#[test]
fn test_validate_unknown_country_passes_generic() {
    let (valid, err, _) = validate_phone_input("99912345678");
    assert!(valid);
    assert!(err.is_none());
}

#[test]
fn test_validate_uae_trunk_stripped_valid() {
    // 971 + 0 + 501234567 -> trunk stripped -> 971501234567 (9 local digits, starts with 5)
    let (valid, err, normalized) = validate_phone_input("9710501234567");
    assert!(valid, "UAE trunk should be stripped: {:?}", err);
    assert_eq!(normalized, "971501234567");
}
