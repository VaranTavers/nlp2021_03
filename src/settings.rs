use regex::Regex;

pub struct Settings {
    pub sep: Regex,
    pub ignore: Regex,
    pub window_size: i32,
}

impl Settings {
    pub fn new(window_size: i32) -> Result<Settings, regex::Error> {
        Ok(Settings {
            sep: Regex::new("[,\\-:\"\'();\t]+")?,
            ignore: Regex::new("[^a-z .]+")?,
            window_size
        })
    }
}