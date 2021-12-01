use std::error;
use std::io::BufReader;
use std::fs::{self, File};

pub struct Input {
    day: String,
}

impl Input {
    pub fn new(day: &str) -> Input {
        Input { day: String::from(day) }
    }

    pub fn as_string(&self) -> Result<String, String> {
        fs::read_to_string(format!("inputs/{}.txt", self.day))
            .or_else(|e| Err(format!("{}", e)))
    }

    pub fn as_stream(&self) -> Result<BufReader<File>, Box<dyn error::Error>> {
        Ok(BufReader::new(
            fs::File::open(format!("inputs/{}.txt", self.day))?
        ))
    }
}
