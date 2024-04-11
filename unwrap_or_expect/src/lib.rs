pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

fn msg(msg: String) -> String {
    format!{"Not found: {}", msg}
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap(),
        Security::High => server.expect("ERROR: program stops"),
        Security::Medium => server.unwrap_or("WARNING: check the server".to_string()),
        Security::Low => server.unwrap_or_else(msg),
        Security::BlockServer => server.unwrap_err()
    }
}