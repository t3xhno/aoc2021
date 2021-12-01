use std::fs;

pub fn as_string(day: &str) -> Result<String, String> {
    fs::read_to_string(format!("inputs/{}.txt", day))
        .or_else(|e| Err(format!("{}", e)))
}