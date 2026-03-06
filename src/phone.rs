/// Normalize a phone number to kwtSMS-accepted format (digits only, international format).
///
/// 1. Coerce to string
/// 2. Trim whitespace
/// 3. Convert Arabic-Indic (U+0660-U+0669) and Extended Arabic-Indic (U+06F0-U+06F9) digits to Latin
/// 4. Strip all non-digit characters
/// 5. Strip leading zeros
pub fn normalize_phone(phone: &str) -> String {
    let trimmed = phone.trim();

    let converted: String = trimmed
        .chars()
        .map(|c| match c {
            // Arabic-Indic digits U+0660-U+0669
            '\u{0660}'..='\u{0669}' => (b'0' + (c as u32 - 0x0660) as u8) as char,
            // Extended Arabic-Indic / Persian digits U+06F0-U+06F9
            '\u{06F0}'..='\u{06F9}' => (b'0' + (c as u32 - 0x06F0) as u8) as char,
            _ => c,
        })
        .collect();

    let digits: String = converted.chars().filter(|c| c.is_ascii_digit()).collect();

    digits.trim_start_matches('0').to_string()
}

/// Validate a phone number input.
///
/// Returns `(is_valid, error_message, normalized)`.
/// - Empty/blank -> error
/// - Contains '@' -> email error
/// - No digits after normalization -> error
/// - Less than 7 digits -> too short
/// - More than 15 digits -> too long
pub fn validate_phone_input(phone: &str) -> (bool, Option<String>, String) {
    let trimmed = phone.trim();

    if trimmed.is_empty() {
        return (
            false,
            Some("Phone number is required".to_string()),
            String::new(),
        );
    }

    if trimmed.contains('@') {
        return (
            false,
            Some(format!(
                "'{}' is an email address, not a phone number",
                trimmed
            )),
            String::new(),
        );
    }

    let normalized = normalize_phone(trimmed);

    if normalized.is_empty() {
        return (
            false,
            Some(format!(
                "'{}' is not a valid phone number, no digits found",
                trimmed
            )),
            String::new(),
        );
    }

    let digit_count = normalized.len();

    if digit_count < 7 {
        return (
            false,
            Some(format!(
                "'{}' is too short ({} digits, minimum is 7)",
                trimmed, digit_count
            )),
            normalized,
        );
    }

    if digit_count > 15 {
        return (
            false,
            Some(format!(
                "'{}' is too long ({} digits, maximum is 15)",
                trimmed, digit_count
            )),
            normalized,
        );
    }

    (true, None, normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    // normalize_phone tests

    #[test]
    fn test_normalize_plain_number() {
        assert_eq!(normalize_phone("96598765432"), "96598765432");
    }

    #[test]
    fn test_normalize_plus_prefix() {
        assert_eq!(normalize_phone("+96598765432"), "96598765432");
    }

    #[test]
    fn test_normalize_double_zero_prefix() {
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
    fn test_normalize_parentheses() {
        assert_eq!(normalize_phone("(965) 98765432"), "96598765432");
    }

    #[test]
    fn test_normalize_arabic_digits() {
        assert_eq!(normalize_phone("\u{0669}\u{0666}\u{0665}\u{0669}\u{0668}\u{0667}\u{0666}\u{0665}\u{0664}\u{0663}\u{0662}"), "96598765432");
    }

    #[test]
    fn test_normalize_persian_digits() {
        assert_eq!(normalize_phone("\u{06F9}\u{06F6}\u{06F5}\u{06F9}\u{06F8}\u{06F7}\u{06F6}\u{06F5}\u{06F4}\u{06F3}\u{06F2}"), "96598765432");
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
    fn test_normalize_leading_zeros() {
        assert_eq!(normalize_phone("0098765432"), "98765432");
    }

    #[test]
    fn test_normalize_mixed() {
        assert_eq!(normalize_phone("+00 965-9876-5432"), "96598765432");
    }

    // validate_phone_input tests

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
        assert!(err.unwrap().contains("email address"));
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
        assert!(err.unwrap().contains("too short"));
        assert_eq!(normalized, "123456");
    }

    #[test]
    fn test_validate_too_long() {
        let (valid, err, _) = validate_phone_input("1234567890123456");
        assert!(!valid);
        assert!(err.unwrap().contains("too long"));
    }

    #[test]
    fn test_validate_minimum_valid() {
        let (valid, err, normalized) = validate_phone_input("1234567");
        assert!(valid);
        assert!(err.is_none());
        assert_eq!(normalized, "1234567");
    }

    #[test]
    fn test_validate_maximum_valid() {
        let (valid, err, normalized) = validate_phone_input("123456789012345");
        assert!(valid);
        assert!(err.is_none());
        assert_eq!(normalized, "123456789012345");
    }

    #[test]
    fn test_validate_arabic_digits() {
        // Arabic-Indic for 96598765432
        let (valid, err, normalized) = validate_phone_input("\u{0669}\u{0666}\u{0665}\u{0669}\u{0668}\u{0667}\u{0666}\u{0665}\u{0664}\u{0663}\u{0662}");
        assert!(valid);
        assert!(err.is_none());
        assert_eq!(normalized, "96598765432");
    }
}
