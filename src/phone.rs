/// Phone number validation rules by country code.
/// Validates local number length and mobile starting digits.
///
/// Sources (verified across 3+ per country):
/// [1] ITU-T E.164 / National Numbering Plans (itu.int)
/// [2] Wikipedia "Telephone numbers in [Country]" articles
/// [3] HowToCallAbroad.com country dialing guides
/// [4] CountryCode.com country format pages
///
/// `local_lengths`: valid digit count(s) AFTER country code
/// `mobile_start_digits`: valid first character(s) of the local number (empty = any)
///
/// Countries not listed here pass through with generic E.164 validation (7-15 digits).
pub struct PhoneRule {
    pub country_code: &'static str,
    pub local_lengths: &'static [u8],
    pub mobile_start_digits: &'static [char],
    pub country_name: &'static str,
}

pub static PHONE_RULES: &[PhoneRule] = &[
    // === GCC ===
    PhoneRule {
        country_code: "965",
        local_lengths: &[8],
        mobile_start_digits: &['4', '5', '6', '9'],
        country_name: "Kuwait",
    },
    PhoneRule {
        country_code: "966",
        local_lengths: &[9],
        mobile_start_digits: &['5'],
        country_name: "Saudi Arabia",
    },
    PhoneRule {
        country_code: "971",
        local_lengths: &[9],
        mobile_start_digits: &['5'],
        country_name: "UAE",
    },
    PhoneRule {
        country_code: "973",
        local_lengths: &[8],
        mobile_start_digits: &['3', '6'],
        country_name: "Bahrain",
    },
    PhoneRule {
        country_code: "974",
        local_lengths: &[8],
        mobile_start_digits: &['3', '5', '6', '7'],
        country_name: "Qatar",
    },
    PhoneRule {
        country_code: "968",
        local_lengths: &[8],
        mobile_start_digits: &['7', '9'],
        country_name: "Oman",
    },
    // === Levant ===
    PhoneRule {
        country_code: "962",
        local_lengths: &[9],
        mobile_start_digits: &['7'],
        country_name: "Jordan",
    },
    PhoneRule {
        country_code: "961",
        local_lengths: &[7, 8],
        mobile_start_digits: &['3', '7', '8'],
        country_name: "Lebanon",
    },
    PhoneRule {
        country_code: "970",
        local_lengths: &[9],
        mobile_start_digits: &['5'],
        country_name: "Palestine",
    },
    PhoneRule {
        country_code: "964",
        local_lengths: &[10],
        mobile_start_digits: &['7'],
        country_name: "Iraq",
    },
    PhoneRule {
        country_code: "963",
        local_lengths: &[9],
        mobile_start_digits: &['9'],
        country_name: "Syria",
    },
    // === Other Arab ===
    PhoneRule {
        country_code: "967",
        local_lengths: &[9],
        mobile_start_digits: &['7'],
        country_name: "Yemen",
    },
    PhoneRule {
        country_code: "20",
        local_lengths: &[10],
        mobile_start_digits: &['1'],
        country_name: "Egypt",
    },
    PhoneRule {
        country_code: "218",
        local_lengths: &[9],
        mobile_start_digits: &['9'],
        country_name: "Libya",
    },
    PhoneRule {
        country_code: "216",
        local_lengths: &[8],
        mobile_start_digits: &['2', '4', '5', '9'],
        country_name: "Tunisia",
    },
    PhoneRule {
        country_code: "212",
        local_lengths: &[9],
        mobile_start_digits: &['6', '7'],
        country_name: "Morocco",
    },
    PhoneRule {
        country_code: "213",
        local_lengths: &[9],
        mobile_start_digits: &['5', '6', '7'],
        country_name: "Algeria",
    },
    PhoneRule {
        country_code: "249",
        local_lengths: &[9],
        mobile_start_digits: &['9'],
        country_name: "Sudan",
    },
    // === Non-Arab Middle East ===
    PhoneRule {
        country_code: "98",
        local_lengths: &[10],
        mobile_start_digits: &['9'],
        country_name: "Iran",
    },
    PhoneRule {
        country_code: "90",
        local_lengths: &[10],
        mobile_start_digits: &['5'],
        country_name: "Turkey",
    },
    PhoneRule {
        country_code: "972",
        local_lengths: &[9],
        mobile_start_digits: &['5'],
        country_name: "Israel",
    },
    // === South Asia ===
    PhoneRule {
        country_code: "91",
        local_lengths: &[10],
        mobile_start_digits: &['6', '7', '8', '9'],
        country_name: "India",
    },
    PhoneRule {
        country_code: "92",
        local_lengths: &[10],
        mobile_start_digits: &['3'],
        country_name: "Pakistan",
    },
    PhoneRule {
        country_code: "880",
        local_lengths: &[10],
        mobile_start_digits: &['1'],
        country_name: "Bangladesh",
    },
    PhoneRule {
        country_code: "94",
        local_lengths: &[9],
        mobile_start_digits: &['7'],
        country_name: "Sri Lanka",
    },
    PhoneRule {
        country_code: "960",
        local_lengths: &[7],
        mobile_start_digits: &['7', '9'],
        country_name: "Maldives",
    },
    // === East Asia ===
    PhoneRule {
        country_code: "86",
        local_lengths: &[11],
        mobile_start_digits: &['1'],
        country_name: "China",
    },
    PhoneRule {
        country_code: "81",
        local_lengths: &[10],
        mobile_start_digits: &['7', '8', '9'],
        country_name: "Japan",
    },
    PhoneRule {
        country_code: "82",
        local_lengths: &[10],
        mobile_start_digits: &['1'],
        country_name: "South Korea",
    },
    PhoneRule {
        country_code: "886",
        local_lengths: &[9],
        mobile_start_digits: &['9'],
        country_name: "Taiwan",
    },
    // === Southeast Asia ===
    PhoneRule {
        country_code: "65",
        local_lengths: &[8],
        mobile_start_digits: &['8', '9'],
        country_name: "Singapore",
    },
    PhoneRule {
        country_code: "60",
        local_lengths: &[9, 10],
        mobile_start_digits: &['1'],
        country_name: "Malaysia",
    },
    PhoneRule {
        country_code: "62",
        local_lengths: &[9, 10, 11, 12],
        mobile_start_digits: &['8'],
        country_name: "Indonesia",
    },
    PhoneRule {
        country_code: "63",
        local_lengths: &[10],
        mobile_start_digits: &['9'],
        country_name: "Philippines",
    },
    PhoneRule {
        country_code: "66",
        local_lengths: &[9],
        mobile_start_digits: &['6', '8', '9'],
        country_name: "Thailand",
    },
    PhoneRule {
        country_code: "84",
        local_lengths: &[9],
        mobile_start_digits: &['3', '5', '7', '8', '9'],
        country_name: "Vietnam",
    },
    PhoneRule {
        country_code: "95",
        local_lengths: &[9],
        mobile_start_digits: &['9'],
        country_name: "Myanmar",
    },
    PhoneRule {
        country_code: "855",
        local_lengths: &[8, 9],
        mobile_start_digits: &['1', '6', '7', '8', '9'],
        country_name: "Cambodia",
    },
    PhoneRule {
        country_code: "976",
        local_lengths: &[8],
        mobile_start_digits: &['6', '8', '9'],
        country_name: "Mongolia",
    },
    // === Europe ===
    PhoneRule {
        country_code: "44",
        local_lengths: &[10],
        mobile_start_digits: &['7'],
        country_name: "UK",
    },
    PhoneRule {
        country_code: "33",
        local_lengths: &[9],
        mobile_start_digits: &['6', '7'],
        country_name: "France",
    },
    PhoneRule {
        country_code: "49",
        local_lengths: &[10, 11],
        mobile_start_digits: &['1'],
        country_name: "Germany",
    },
    PhoneRule {
        country_code: "39",
        local_lengths: &[10],
        mobile_start_digits: &['3'],
        country_name: "Italy",
    },
    PhoneRule {
        country_code: "34",
        local_lengths: &[9],
        mobile_start_digits: &['6', '7'],
        country_name: "Spain",
    },
    PhoneRule {
        country_code: "31",
        local_lengths: &[9],
        mobile_start_digits: &['6'],
        country_name: "Netherlands",
    },
    PhoneRule {
        country_code: "32",
        local_lengths: &[9],
        mobile_start_digits: &[],
        country_name: "Belgium",
    },
    PhoneRule {
        country_code: "41",
        local_lengths: &[9],
        mobile_start_digits: &['7'],
        country_name: "Switzerland",
    },
    PhoneRule {
        country_code: "43",
        local_lengths: &[10],
        mobile_start_digits: &['6'],
        country_name: "Austria",
    },
    PhoneRule {
        country_code: "47",
        local_lengths: &[8],
        mobile_start_digits: &['4', '9'],
        country_name: "Norway",
    },
    PhoneRule {
        country_code: "48",
        local_lengths: &[9],
        mobile_start_digits: &[],
        country_name: "Poland",
    },
    PhoneRule {
        country_code: "30",
        local_lengths: &[10],
        mobile_start_digits: &['6'],
        country_name: "Greece",
    },
    PhoneRule {
        country_code: "420",
        local_lengths: &[9],
        mobile_start_digits: &['6', '7'],
        country_name: "Czech Republic",
    },
    PhoneRule {
        country_code: "46",
        local_lengths: &[9],
        mobile_start_digits: &['7'],
        country_name: "Sweden",
    },
    PhoneRule {
        country_code: "45",
        local_lengths: &[8],
        mobile_start_digits: &[],
        country_name: "Denmark",
    },
    PhoneRule {
        country_code: "40",
        local_lengths: &[9],
        mobile_start_digits: &['7'],
        country_name: "Romania",
    },
    PhoneRule {
        country_code: "36",
        local_lengths: &[9],
        mobile_start_digits: &[],
        country_name: "Hungary",
    },
    PhoneRule {
        country_code: "380",
        local_lengths: &[9],
        mobile_start_digits: &[],
        country_name: "Ukraine",
    },
    // === Americas ===
    PhoneRule {
        country_code: "1",
        local_lengths: &[10],
        mobile_start_digits: &[],
        country_name: "USA/Canada",
    },
    PhoneRule {
        country_code: "52",
        local_lengths: &[10],
        mobile_start_digits: &[],
        country_name: "Mexico",
    },
    PhoneRule {
        country_code: "55",
        local_lengths: &[11],
        mobile_start_digits: &[],
        country_name: "Brazil",
    },
    PhoneRule {
        country_code: "57",
        local_lengths: &[10],
        mobile_start_digits: &['3'],
        country_name: "Colombia",
    },
    PhoneRule {
        country_code: "54",
        local_lengths: &[10],
        mobile_start_digits: &['9'],
        country_name: "Argentina",
    },
    PhoneRule {
        country_code: "56",
        local_lengths: &[9],
        mobile_start_digits: &['9'],
        country_name: "Chile",
    },
    PhoneRule {
        country_code: "58",
        local_lengths: &[10],
        mobile_start_digits: &['4'],
        country_name: "Venezuela",
    },
    PhoneRule {
        country_code: "51",
        local_lengths: &[9],
        mobile_start_digits: &['9'],
        country_name: "Peru",
    },
    PhoneRule {
        country_code: "593",
        local_lengths: &[9],
        mobile_start_digits: &['9'],
        country_name: "Ecuador",
    },
    PhoneRule {
        country_code: "53",
        local_lengths: &[8],
        mobile_start_digits: &['5', '6'],
        country_name: "Cuba",
    },
    // === Africa ===
    PhoneRule {
        country_code: "27",
        local_lengths: &[9],
        mobile_start_digits: &['6', '7', '8'],
        country_name: "South Africa",
    },
    PhoneRule {
        country_code: "234",
        local_lengths: &[10],
        mobile_start_digits: &['7', '8', '9'],
        country_name: "Nigeria",
    },
    PhoneRule {
        country_code: "254",
        local_lengths: &[9],
        mobile_start_digits: &['1', '7'],
        country_name: "Kenya",
    },
    PhoneRule {
        country_code: "233",
        local_lengths: &[9],
        mobile_start_digits: &['2', '5'],
        country_name: "Ghana",
    },
    PhoneRule {
        country_code: "251",
        local_lengths: &[9],
        mobile_start_digits: &['7', '9'],
        country_name: "Ethiopia",
    },
    PhoneRule {
        country_code: "255",
        local_lengths: &[9],
        mobile_start_digits: &['6', '7'],
        country_name: "Tanzania",
    },
    PhoneRule {
        country_code: "256",
        local_lengths: &[9],
        mobile_start_digits: &['7'],
        country_name: "Uganda",
    },
    PhoneRule {
        country_code: "237",
        local_lengths: &[9],
        mobile_start_digits: &['6'],
        country_name: "Cameroon",
    },
    PhoneRule {
        country_code: "225",
        local_lengths: &[10],
        mobile_start_digits: &[],
        country_name: "Ivory Coast",
    },
    PhoneRule {
        country_code: "221",
        local_lengths: &[9],
        mobile_start_digits: &['7'],
        country_name: "Senegal",
    },
    PhoneRule {
        country_code: "252",
        local_lengths: &[9],
        mobile_start_digits: &['6', '7'],
        country_name: "Somalia",
    },
    PhoneRule {
        country_code: "250",
        local_lengths: &[9],
        mobile_start_digits: &['7'],
        country_name: "Rwanda",
    },
    // === Oceania ===
    PhoneRule {
        country_code: "61",
        local_lengths: &[9],
        mobile_start_digits: &['4'],
        country_name: "Australia",
    },
    PhoneRule {
        country_code: "64",
        local_lengths: &[8, 9, 10],
        mobile_start_digits: &['2'],
        country_name: "New Zealand",
    },
];

/// Find the country code prefix from a normalized phone number.
/// Tries 3-digit codes first, then 2-digit, then 1-digit (longest match wins).
pub fn find_country_code(normalized: &str) -> Option<&'static str> {
    // Try 3-digit, then 2-digit, then 1-digit (longest match wins)
    for len in [3, 2, 1] {
        if normalized.len() >= len {
            let prefix = &normalized[..len];
            if let Some(rule) = PHONE_RULES.iter().find(|r| r.country_code == prefix) {
                return Some(rule.country_code);
            }
        }
    }
    None
}

/// Get the country name for a country code, or "+{cc}" if not found.
pub fn country_name_for_code(cc: &str) -> String {
    PHONE_RULES
        .iter()
        .find(|r| r.country_code == cc)
        .map(|r| r.country_name.to_string())
        .unwrap_or_else(|| format!("+{}", cc))
}

/// Validate a normalized phone number against country-specific format rules.
/// Checks local number length and mobile starting digits.
/// Numbers with no matching country rules pass through (generic E.164 only).
pub fn validate_phone_format(normalized: &str) -> (bool, Option<String>) {
    let cc = match find_country_code(normalized) {
        Some(cc) => cc,
        None => return (true, None),
    };

    let rule = match PHONE_RULES.iter().find(|r| r.country_code == cc) {
        Some(r) => r,
        None => return (true, None),
    };

    let local = &normalized[cc.len()..];
    let country = rule.country_name;

    // Check local number length
    if !rule.local_lengths.contains(&(local.len() as u8)) {
        let expected: Vec<String> = rule.local_lengths.iter().map(|l| l.to_string()).collect();
        return (
            false,
            Some(format!(
                "Invalid {} number: expected {} digits after +{}, got {}",
                country,
                expected.join(" or "),
                cc,
                local.len()
            )),
        );
    }

    // Check mobile starting digits (if rules exist for this country)
    if !rule.mobile_start_digits.is_empty() {
        let first_char = local.chars().next();
        let has_valid_prefix = first_char
            .map(|c| rule.mobile_start_digits.contains(&c))
            .unwrap_or(false);
        if !has_valid_prefix {
            let prefixes: Vec<String> = rule
                .mobile_start_digits
                .iter()
                .map(|c| c.to_string())
                .collect();
            return (
                false,
                Some(format!(
                    "Invalid {} mobile number: after +{} must start with {}",
                    country,
                    cc,
                    prefixes.join(", ")
                )),
            );
        }
    }

    (true, None)
}

/// Normalize a phone number to kwtSMS-accepted format (digits only, international format).
///
/// 1. Trim whitespace
/// 2. Convert Arabic-Indic and Extended Arabic-Indic digits to Latin
/// 3. Strip all non-digit characters
/// 4. Strip leading zeros
/// 5. Strip domestic trunk prefix (leading 0 after country code, e.g. 9660559... -> 966559...)
pub fn normalize_phone(phone: &str) -> String {
    let trimmed = phone.trim();

    let converted: String = trimmed
        .chars()
        .map(|c| match c {
            '\u{0660}'..='\u{0669}' => (b'0' + (c as u32 - 0x0660) as u8) as char,
            '\u{06F0}'..='\u{06F9}' => (b'0' + (c as u32 - 0x06F0) as u8) as char,
            _ => c,
        })
        .collect();

    let digits: String = converted.chars().filter(|c| c.is_ascii_digit()).collect();

    let stripped = digits.trim_start_matches('0');

    if stripped.is_empty() {
        return String::new();
    }

    // Strip domestic trunk prefix: leading 0 after country code
    // e.g. 9660559... -> 966559..., 97105x -> 9715x, 20010x -> 2010x
    if let Some(cc) = find_country_code(stripped) {
        let local = &stripped[cc.len()..];
        if local.starts_with('0') {
            let local_stripped = local.trim_start_matches('0');
            if !local_stripped.is_empty() {
                return format!("{}{}", cc, local_stripped);
            }
        }
    }

    stripped.to_string()
}

/// Validate a phone number input.
///
/// Returns `(is_valid, error_message, normalized)`.
/// - Empty/blank -> error
/// - Contains '@' -> email error
/// - No digits after normalization -> error
/// - Less than 7 digits -> too short
/// - More than 15 digits -> too long
/// - Country-specific format check (length + mobile prefix)
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

    // Country-specific format validation (length + mobile prefix)
    let (format_valid, format_error) = validate_phone_format(&normalized);
    if !format_valid {
        return (false, format_error, normalized);
    }

    (true, None, normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== normalize_phone tests =====

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
        assert_eq!(
            normalize_phone("\u{0669}\u{0666}\u{0665}\u{0669}\u{0668}\u{0667}\u{0666}\u{0665}\u{0664}\u{0663}\u{0662}"),
            "96598765432"
        );
    }

    #[test]
    fn test_normalize_persian_digits() {
        assert_eq!(
            normalize_phone("\u{06F9}\u{06F6}\u{06F5}\u{06F9}\u{06F8}\u{06F7}\u{06F6}\u{06F5}\u{06F4}\u{06F3}\u{06F2}"),
            "96598765432"
        );
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

    // ===== Domestic trunk prefix stripping =====

    #[test]
    fn test_normalize_saudi_trunk_prefix() {
        // 9660559... -> 966559...
        assert_eq!(normalize_phone("9660559876543"), "966559876543");
    }

    #[test]
    fn test_normalize_saudi_with_plus_trunk() {
        assert_eq!(normalize_phone("+9660559876543"), "966559876543");
    }

    #[test]
    fn test_normalize_saudi_with_00_trunk() {
        assert_eq!(normalize_phone("009660559876543"), "966559876543");
    }

    #[test]
    fn test_normalize_uae_trunk_prefix() {
        // 9710501234567 -> 971501234567
        assert_eq!(normalize_phone("9710501234567"), "971501234567");
    }

    #[test]
    fn test_normalize_egypt_trunk_prefix() {
        // 20010x -> 2010x
        assert_eq!(normalize_phone("20010123456789"), "2010123456789");
    }

    #[test]
    fn test_normalize_no_trunk_no_change() {
        // Already correct, no trunk prefix
        assert_eq!(normalize_phone("966559876543"), "966559876543");
    }

    #[test]
    fn test_normalize_kuwait_no_trunk() {
        // Kuwait numbers don't have trunk prefixes
        assert_eq!(normalize_phone("96598765432"), "96598765432");
    }

    // ===== find_country_code tests =====

    #[test]
    fn test_find_cc_kuwait() {
        assert_eq!(find_country_code("96598765432"), Some("965"));
    }

    #[test]
    fn test_find_cc_usa() {
        assert_eq!(find_country_code("12025551234"), Some("1"));
    }

    #[test]
    fn test_find_cc_egypt() {
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

    // ===== validate_phone_format tests =====

    #[test]
    fn test_format_kuwait_valid() {
        let (valid, err) = validate_phone_format("96598765432");
        assert!(valid);
        assert!(err.is_none());
    }

    #[test]
    fn test_format_kuwait_wrong_length() {
        let (valid, err) = validate_phone_format("9659876543"); // 7 local digits instead of 8
        assert!(!valid);
        assert!(err.unwrap().contains("expected 8 digits"));
    }

    #[test]
    fn test_format_kuwait_wrong_prefix() {
        let (valid, err) = validate_phone_format("96512345678"); // starts with 1, not 4/5/6/9
        assert!(!valid);
        assert!(err.unwrap().contains("must start with"));
    }

    #[test]
    fn test_format_saudi_valid() {
        let (valid, err) = validate_phone_format("966559876543");
        assert!(valid);
        assert!(err.is_none());
    }

    #[test]
    fn test_format_saudi_wrong_prefix() {
        let (valid, err) = validate_phone_format("966159876543"); // starts with 1, not 5
        assert!(!valid);
        assert!(err.unwrap().contains("must start with 5"));
    }

    #[test]
    fn test_format_unknown_country_passes() {
        let (valid, err) = validate_phone_format("99912345678");
        assert!(valid);
        assert!(err.is_none());
    }

    #[test]
    fn test_format_usa_valid() {
        let (valid, err) = validate_phone_format("12025551234");
        assert!(valid);
        assert!(err.is_none());
    }

    #[test]
    fn test_format_belgium_no_prefix_check() {
        // Belgium has no mobile_start_digits, only length check
        let (valid, err) = validate_phone_format("32412345678");
        assert!(valid);
        assert!(err.is_none());
    }

    // ===== validate_phone_input tests (with country format) =====

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
        // Use 999xxxx (no country rule) for generic E.164 test
        let (valid, err, normalized) = validate_phone_input("9991234");
        assert!(valid);
        assert!(err.is_none());
        assert_eq!(normalized, "9991234");
    }

    #[test]
    fn test_validate_maximum_valid() {
        // Use 999... (no country rule) for generic E.164 test
        let (valid, err, normalized) = validate_phone_input("999456789012345");
        assert!(valid);
        assert!(err.is_none());
        assert_eq!(normalized, "999456789012345");
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
    fn test_validate_saudi_trunk_stripped_and_valid() {
        // 9660559876543 -> normalizes to 966559876543, then validates Saudi format
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
    fn test_validate_kuwait_all_valid_prefixes() {
        // 4x, 5x, 6x, 9x are valid Kuwait mobile prefixes
        for prefix in ["4", "5", "6", "9"] {
            let number = format!("965{}1234567", prefix);
            let (valid, err, _) = validate_phone_input(&number);
            assert!(
                valid,
                "Kuwait prefix {} should be valid, got: {:?}",
                prefix, err
            );
        }
    }

    #[test]
    fn test_validate_uae_valid() {
        let (valid, err, _) = validate_phone_input("971501234567");
        assert!(valid);
        assert!(err.is_none());
    }

    #[test]
    fn test_validate_unknown_country_passes_generic() {
        // Unknown country code, passes generic E.164 (7-15 digits)
        let (valid, err, _) = validate_phone_input("99912345678");
        assert!(valid);
        assert!(err.is_none());
    }
}
