use std::fs;

#[derive(Debug)]
pub struct Config {
    pub smpp_host: String,
    pub system_id: String,
    pub password: String,
    pub system_type: String,
    pub sender_id: String,
}

pub fn load_config(path: &str) -> std::io::Result<Config> {
    let contents = fs::read_to_string(path)?;

    let mut smpp_host = String::new();
    let mut system_id = String::new();
    let mut password = String::new();
    let mut system_type = String::new();
    let mut sender_id = String::new();


    for line in contents.lines() {
        let line = line.trim();

        if let Some(value) = line.strip_prefix("system_id = ") {
            system_id = value.trim_matches('"').to_string();
        } else if let Some(value) = line.strip_prefix("password = ") {
            password = value.trim_matches('"').to_string();
        } else if let Some(value) = line.strip_prefix("system_type = ") {
            system_type = value.trim_matches('"').to_string();
        } else if let Some(value) = line.strip_prefix("sender_id = ") {
            sender_id = value.trim_matches('"').to_string();
        } else if let Some(value) = line.strip_prefix("smpp_host = ") {
            smpp_host = value.trim_matches('"').to_string();
        }
    }

    Ok(Config {
        smpp_host,
        system_id,
        password,
        system_type,
        sender_id,
    })
}
