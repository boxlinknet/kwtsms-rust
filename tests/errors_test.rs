use kwtsms::errors::{enrich_error, KwtSmsError, API_ERRORS};

#[test]
fn test_all_33_error_codes_have_actions() {
    let codes = vec![
        "ERR001",
        "ERR002",
        "ERR003",
        "ERR004",
        "ERR005",
        "ERR006",
        "ERR007",
        "ERR008",
        "ERR009",
        "ERR010",
        "ERR011",
        "ERR012",
        "ERR013",
        "ERR019",
        "ERR020",
        "ERR021",
        "ERR022",
        "ERR023",
        "ERR024",
        "ERR025",
        "ERR026",
        "ERR027",
        "ERR028",
        "ERR029",
        "ERR030",
        "ERR031",
        "ERR032",
        "ERR033",
        "ERR_INVALID_INPUT",
    ];
    for code in &codes {
        assert!(
            API_ERRORS.contains_key(code),
            "Missing error code: {}",
            code
        );
        assert!(
            !API_ERRORS.get(code).unwrap().is_empty(),
            "Empty action for: {}",
            code
        );
    }
}

#[test]
fn test_enrich_err003() {
    let mut data = serde_json::json!({
        "result": "ERROR",
        "code": "ERR003",
        "description": "Authentication error"
    });
    enrich_error(&mut data);
    let action = data["action"].as_str().unwrap();
    assert!(action.contains("KWTSMS_USERNAME"));
    assert!(action.contains("KWTSMS_PASSWORD"));
}

#[test]
fn test_enrich_err010() {
    let mut data = serde_json::json!({
        "result": "ERROR",
        "code": "ERR010",
        "description": "Zero balance"
    });
    enrich_error(&mut data);
    assert!(data["action"].as_str().unwrap().contains("kwtsms.com"));
}

#[test]
fn test_enrich_err024() {
    let mut data = serde_json::json!({
        "result": "ERROR",
        "code": "ERR024",
        "description": "IP not whitelisted"
    });
    enrich_error(&mut data);
    assert!(data["action"].as_str().unwrap().contains("IP"));
}

#[test]
fn test_enrich_err026() {
    let mut data = serde_json::json!({
        "result": "ERROR",
        "code": "ERR026",
        "description": "Country not activated"
    });
    enrich_error(&mut data);
    assert!(data["action"].as_str().unwrap().contains("country"));
}

#[test]
fn test_enrich_err028() {
    let mut data = serde_json::json!({
        "result": "ERROR",
        "code": "ERR028",
        "description": "Rate limit"
    });
    enrich_error(&mut data);
    assert!(data["action"].as_str().unwrap().contains("15 seconds"));
}

#[test]
fn test_enrich_err008() {
    let mut data = serde_json::json!({
        "result": "ERROR",
        "code": "ERR008",
        "description": "Sender ID banned"
    });
    enrich_error(&mut data);
    assert!(data["action"].as_str().unwrap().contains("sender"));
}

#[test]
fn test_enrich_unknown_code_no_action() {
    let mut data = serde_json::json!({
        "result": "ERROR",
        "code": "ERR999",
        "description": "Unknown error"
    });
    enrich_error(&mut data);
    assert!(data.get("action").is_none());
}

#[test]
fn test_enrich_ok_no_action() {
    let mut data = serde_json::json!({
        "result": "OK",
        "available": 100
    });
    enrich_error(&mut data);
    assert!(data.get("action").is_none());
}

#[test]
fn test_error_display_network() {
    let e = KwtSmsError::Network("connection refused".to_string());
    let s = e.to_string();
    assert!(s.contains("connection refused"));
}

#[test]
fn test_error_display_api() {
    let e = KwtSmsError::Api {
        code: "ERR003".to_string(),
        description: "Auth error".to_string(),
        action: "Check credentials".to_string(),
    };
    let s = e.to_string();
    assert!(s.contains("ERR003"));
    assert!(s.contains("Check credentials"));
}

#[test]
fn test_error_display_invalid_input() {
    let e = KwtSmsError::InvalidInput("empty phone".to_string());
    assert!(e.to_string().contains("empty phone"));
}

#[test]
fn test_enrich_all_error_codes() {
    for (code, expected_action) in API_ERRORS.iter() {
        let mut data = serde_json::json!({
            "result": "ERROR",
            "code": code,
            "description": "test"
        });
        enrich_error(&mut data);
        let action = data["action"].as_str().unwrap();
        assert_eq!(action, *expected_action, "Action mismatch for {}", code);
    }
}
