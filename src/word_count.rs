extern crate regex;

use regex::Regex;

use std::collections::HashMap;

pub struct WordCounter {
    sep: Regex,
    ignore: Regex,
}

impl WordCounter {
    pub fn new() -> Result<WordCounter, regex::Error> {
        Ok(WordCounter {
            sep: Regex::new("[.,\\-:\"\'()!?;\t]+")?,
            ignore: Regex::new("[^a-z]+")?,
        })
    }
    pub fn word_count(&self, text: &str) -> HashMap<String, i32> {
        let text = text.to_lowercase();
        let text = self.sep.replace_all(&text, " ");
        let text = self.ignore.replace_all(&text, "");
        let mut res = HashMap::new();

        println!("{}", text);
        for word in text.split(' ') {
            let entry = res.entry(word.to_string()).or_insert(0);
            *entry += 1;
        }

        res
    }

    pub fn word_count_existing(&self, text: &str, res: &mut HashMap<String, i32>) {
        let text = text.to_lowercase();
        let text = self.sep.replace_all(&text, " ");
        let text = self.ignore.replace_all(&text, "");

        println!("{}", text);
        for word in text.split(' ') {
            let entry = res.entry(word.to_string()).or_insert(0);
            *entry += 1;
        }

    }
}
