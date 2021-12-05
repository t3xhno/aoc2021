use std::error;
use std::fs::{self, File};
use std::io::BufReader;

const INPUT_EXT: &'static str = "txt";
const INPUT_PATH: &'static str = "inputs";

pub struct Input {
    day: String,
}

impl Input {
    pub fn new(day: &str) -> Input {
        Input {
            day: String::from(day),
        }
    }

    fn make_path(&self) -> String {
        format!("{}/{}.{}", INPUT_PATH, self.day, INPUT_EXT)
    }

    pub fn as_string(&self) -> Result<String, String> {
        fs::read_to_string(self.make_path()).or_else(|e| Err(format!("{}", e)))
    }

    pub fn as_stream(&self) -> Result<BufReader<File>, Box<dyn error::Error>> {
        Ok(BufReader::new(fs::File::open(format!(
            "inputs/{}.txt",
            self.day
        ))?))
    }
}
