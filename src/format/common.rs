pub fn check_identity(id: &str) {
    let valid = id == "server" || id == "client" || {
        id.len() == 5 && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    };

    if !valid {
        panic!("Invalid identity: '{}'. Must be 'server', 'client', or [a-z0-9]{{5}}", id);
    }
}

pub fn compress_identity(id: &str) -> &str {
    match id {
        "server" => "s",
        "client" => "c",
        _ => id,
    }
}