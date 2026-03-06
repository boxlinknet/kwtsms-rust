use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Load environment variables from a `.env` file.
///
/// Parsing rules:
/// - Skip blank lines and lines starting with `#`
/// - Split on first `=` only (values can contain `=`)
/// - Strip inline `# comments` from unquoted values
/// - Strip one matching outer quote pair (single or double)
/// - Return empty map for missing files (never panic)
/// - Does NOT modify the process environment (read-only)
pub fn load_env_file(path: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let file_path = Path::new(path);
    if !file_path.exists() {
        return map;
    }

    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return map,
    };

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim().to_string();
            let mut value = trimmed[eq_pos + 1..].to_string();

            // Check if value is quoted
            let trimmed_value = value.trim();
            if (trimmed_value.starts_with('"') && trimmed_value.ends_with('"'))
                || (trimmed_value.starts_with('\'') && trimmed_value.ends_with('\''))
            {
                // Strip matching outer quotes
                if trimmed_value.len() >= 2 {
                    value = trimmed_value[1..trimmed_value.len() - 1].to_string();
                }
            } else {
                // Strip inline comments for unquoted values
                if let Some(comment_pos) = value.find('#') {
                    // Only strip if preceded by whitespace
                    if comment_pos > 0 && value[..comment_pos].ends_with(' ') {
                        value = value[..comment_pos].trim_end().to_string();
                    }
                }
                value = value.trim().to_string();
            }

            if !key.is_empty() {
                map.insert(key, value);
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn write_temp_env(content: &str) -> String {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = format!("/tmp/kwtsms_test_env_{}_{}", std::process::id(), id);
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_load_basic() {
        let path = write_temp_env("KEY=value\nOTHER=hello");
        let env = load_env_file(&path);
        assert_eq!(env.get("KEY").unwrap(), "value");
        assert_eq!(env.get("OTHER").unwrap(), "hello");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_load_missing_file() {
        let env = load_env_file("/tmp/nonexistent_env_file_kwtsms");
        assert!(env.is_empty());
    }

    #[test]
    fn test_load_blank_lines_and_comments() {
        let path = write_temp_env("# comment\n\nKEY=value\n# another comment\n\n");
        let env = load_env_file(&path);
        assert_eq!(env.len(), 1);
        assert_eq!(env.get("KEY").unwrap(), "value");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_load_quoted_double() {
        let path = write_temp_env("KEY=\"hello world\"");
        let env = load_env_file(&path);
        assert_eq!(env.get("KEY").unwrap(), "hello world");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_load_quoted_single() {
        let path = write_temp_env("KEY='hello world'");
        let env = load_env_file(&path);
        assert_eq!(env.get("KEY").unwrap(), "hello world");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_load_mismatched_quotes() {
        let path = write_temp_env("KEY=\"hello'");
        let env = load_env_file(&path);
        // Mismatched quotes: not stripped
        assert_eq!(env.get("KEY").unwrap(), "\"hello'");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_load_inline_comment() {
        let path = write_temp_env("KEY=value # this is a comment");
        let env = load_env_file(&path);
        assert_eq!(env.get("KEY").unwrap(), "value");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_load_value_with_equals() {
        let path = write_temp_env("KEY=a=b=c");
        let env = load_env_file(&path);
        assert_eq!(env.get("KEY").unwrap(), "a=b=c");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_load_hash_in_value_no_space() {
        let path = write_temp_env("PASS=p@ss#word!");
        let env = load_env_file(&path);
        assert_eq!(env.get("PASS").unwrap(), "p@ss#word!");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_load_empty_value() {
        let path = write_temp_env("KEY=");
        let env = load_env_file(&path);
        assert_eq!(env.get("KEY").unwrap(), "");
        fs::remove_file(&path).ok();
    }
}
