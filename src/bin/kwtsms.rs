use kwtsms::KwtSms;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let command = args[1].as_str();

    match command {
        "help" | "--help" | "-h" => print_usage(),
        "verify" => cmd_verify(),
        "balance" => cmd_balance(),
        "senderid" => cmd_senderid(),
        "coverage" => cmd_coverage(),
        "send" => cmd_send(&args[2..]),
        "validate" => cmd_validate(&args[2..]),
        "status" => cmd_status(&args[2..]),
        "dlr" => cmd_dlr(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn get_client() -> KwtSms {
    match KwtSms::from_env(None) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create client: {}", e);
            eprintln!("Make sure KWTSMS_USERNAME and KWTSMS_PASSWORD are set in your environment or .env file.");
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("kwtsms - kwtSMS API CLI client");
    println!();
    println!("Usage: kwtsms <command> [arguments]");
    println!();
    println!("Commands:");
    println!("  verify                          Test credentials and show balance");
    println!("  balance                         Show available and purchased credits");
    println!("  senderid                        List registered sender IDs");
    println!("  coverage                        List active country prefixes");
    println!("  send <mobile> <message> [--sender ID]  Send SMS");
    println!("  validate <number> [number ...]  Validate phone numbers");
    println!("  status <msg-id>                 Check message status");
    println!("  dlr <msg-id>                    Get delivery report (international)");
    println!();
    println!("Environment:");
    println!("  KWTSMS_USERNAME   API username");
    println!("  KWTSMS_PASSWORD   API password");
    println!("  KWTSMS_SENDER_ID  Default sender ID (default: KWT-SMS)");
    println!("  KWTSMS_TEST_MODE  1 = test mode (default: 0)");
    println!("  KWTSMS_LOG_FILE   Log file path (default: kwtsms.log)");
}

fn cmd_verify() {
    let client = get_client();
    let result = client.verify();
    if result.ok {
        println!("Credentials OK");
        if let Some(bal) = result.balance {
            println!("Available balance: {}", bal);
        }
        if let Some(pur) = result.purchased {
            println!("Purchased credits: {}", pur);
        }
    } else {
        eprintln!("Verification failed: {}", result.error.unwrap_or_default());
        process::exit(1);
    }
}

fn cmd_balance() {
    let client = get_client();
    let result = client.verify();
    if result.ok {
        if let Some(bal) = result.balance {
            println!("Available: {}", bal);
        }
        if let Some(pur) = result.purchased {
            println!("Purchased: {}", pur);
        }
    } else {
        eprintln!("Error: {}", result.error.unwrap_or_default());
        process::exit(1);
    }
}

fn cmd_senderid() {
    let client = get_client();
    match client.senderids() {
        Ok(data) => {
            let result = data
                .get("result")
                .and_then(|v| v.as_str())
                .unwrap_or("ERROR");
            if result == "OK" {
                if let Some(ids) = data.get("senderids").and_then(|v| v.as_array()) {
                    println!("Sender IDs:");
                    for id in ids {
                        if let Some(s) = id.as_str() {
                            println!("  {}", s);
                        }
                    }
                }
            } else {
                eprintln!(
                    "Error: {}",
                    data.get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error")
                );
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_coverage() {
    let client = get_client();
    match client.coverage() {
        Ok(data) => {
            let result = data
                .get("result")
                .and_then(|v| v.as_str())
                .unwrap_or("ERROR");
            if result == "OK" {
                if let Some(prefixes) = data.get("prefixes").and_then(|v| v.as_array()) {
                    println!("Active country prefixes:");
                    for p in prefixes {
                        if let Some(s) = p.as_str() {
                            println!("  {}", s);
                        }
                    }
                }
            } else {
                eprintln!(
                    "Error: {}",
                    data.get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error")
                );
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_send(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: kwtsms send <mobile> <message> [--sender ID]");
        process::exit(1);
    }

    let mobile = &args[0];
    let message = &args[1];
    let mut sender: Option<&str> = None;

    let mut i = 2;
    while i < args.len() {
        if args[i] == "--sender" && i + 1 < args.len() {
            sender = Some(&args[i + 1]);
            i += 2;
        } else {
            i += 1;
        }
    }

    let client = get_client();

    // Warn about test mode
    if env::var("KWTSMS_TEST_MODE").unwrap_or_default().as_str() == "1" {
        eprintln!("WARNING: Test mode is ON. Messages will be queued but NOT delivered.");
    }

    match client.send_one(mobile, message, sender) {
        Ok(data) => {
            let result = data
                .get("result")
                .and_then(|v| v.as_str())
                .unwrap_or("ERROR");
            if result == "OK" {
                println!("SMS sent successfully.");
                if let Some(id) = data.get("msg-id").and_then(|v| v.as_str()) {
                    println!("Message ID: {}", id);
                }
                if let Some(bal) = data.get("balance-after").and_then(|v| v.as_f64()) {
                    println!("Balance after: {}", bal);
                }
            } else {
                eprintln!(
                    "Error: {}",
                    data.get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error")
                );
                if let Some(action) = data.get("action").and_then(|v| v.as_str()) {
                    eprintln!("Action: {}", action);
                }
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_validate(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: kwtsms validate <number> [number ...]");
        process::exit(1);
    }

    let client = get_client();
    let numbers: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match client.validate(&numbers) {
        Ok(data) => println!(
            "{}",
            serde_json::to_string_pretty(&data).unwrap_or_default()
        ),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_status(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: kwtsms status <msg-id>");
        process::exit(1);
    }

    let client = get_client();
    match client.status(&args[0]) {
        Ok(data) => println!(
            "{}",
            serde_json::to_string_pretty(&data).unwrap_or_default()
        ),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_dlr(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: kwtsms dlr <msg-id>");
        process::exit(1);
    }

    let client = get_client();
    match client.dlr(&args[0]) {
        Ok(data) => println!(
            "{}",
            serde_json::to_string_pretty(&data).unwrap_or_default()
        ),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
