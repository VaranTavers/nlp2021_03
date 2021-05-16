extern crate regex;

use regex::Regex;

use std::collections::HashMap;

use std::io::BufWriter;
use std::io::Write;

use std::fs::File;


pub struct WordCounter {
    sep: Regex,
    ignore: Regex,
}

impl WordCounter {
    pub fn new() -> Result<WordCounter, regex::Error> {
        Ok(WordCounter {
            sep: Regex::new("[.,\\-:\"\'()!?;\t]+")?,
            ignore: Regex::new("[^a-z ]+")?,
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

        for word in text.split(' ') {
            let entry = res.entry(word.to_string()).or_insert(0);
            *entry += 1;
        }

    }
}

fn wc_all_files(folder_name: &str) -> std::io::Result<HashMap<String, i32>> {
    let folder = std::fs::read_dir(folder_name)?;

    let mut dict: HashMap<String, i32> = HashMap::new();
    let wc = WordCounter::new().unwrap();
    for entry in folder {
        let text = std::fs::read_to_string(entry.unwrap().path())?;
        wc.word_count_existing(&text, &mut dict);
    }

    Ok(dict)
}

fn cull_word_map(dict: HashMap<String, i32>, high: usize, low: usize) -> Vec<String> {
    let mut counts = dict.values().cloned().collect::<Vec<i32>>();
    counts.sort_unstable();
    counts.reverse();
    let high_threshold = counts[high];
    let low_threshold = counts[low];

    let words = dict.keys()
                    .filter(|k| dict[*k] >= low_threshold && dict[*k] <= high_threshold)
                    .cloned()
                    .collect::<Vec<String>>();

    words
}

pub fn create_word_index() -> Result<(), std::io::Error> {
    let hmap = wc_all_files("./korpusz")?;
    let words = cull_word_map(hmap, 99, 10099);

    let out = File::create("word_index.csv")?;
    let mut writer = BufWriter::new(out);
    for (i, w) in words.iter().enumerate() {
        if i <= 10000 {
            writer.write_all(format!("{}; {}\n", w, i).as_bytes())?;
        }
    }
    Ok(())
}