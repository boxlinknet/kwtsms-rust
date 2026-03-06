use kwtsms::{normalize_phone, validate_phone_input};

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
    // ٩٦٥٩٨٧٦٥٤٣٢
    assert_eq!(
        normalize_phone("\u{0669}\u{0666}\u{0665}\u{0669}\u{0668}\u{0667}\u{0666}\u{0665}\u{0664}\u{0663}\u{0662}"),
        "96598765432"
    );
}

#[test]
fn test_normalize_persian_digits() {
    // ۹۶۵۹۸۷۶۵۴۳۲
    assert_eq!(
        normalize_phone("\u{06F9}\u{06F6}\u{06F5}\u{06F9}\u{06F8}\u{06F7}\u{06F6}\u{06F5}\u{06F4}\u{06F3}\u{06F2}"),
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
    // Mix of Arabic-Indic and Latin digits
    assert_eq!(normalize_phone("965\u{0669}876\u{0665}432"), "96598765432");
}

#[test]
fn test_normalize_leading_trailing_whitespace() {
    assert_eq!(normalize_phone("  96598765432  "), "96598765432");
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
    let (valid, err, normalized) = validate_phone_input("1234567");
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "1234567");
}

#[test]
fn test_validate_maximum_valid_15_digits() {
    let (valid, err, normalized) = validate_phone_input("123456789012345");
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "123456789012345");
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
    // Contains @ so it's an email check
    assert!(err.unwrap().contains("email address"));
}

#[test]
fn test_validate_only_special_no_at() {
    let (valid, err, _) = validate_phone_input("!#$%^&*");
    assert!(!valid);
    assert!(err.unwrap().contains("no digits found"));
}
