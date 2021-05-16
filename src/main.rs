use std::collections::HashMap;

use std::io::BufWriter;
use std::io::Write;

use std::fs::File;

mod word_count;

fn wc_all_files(folder_name: &str) -> std::io::Result<HashMap<String, i32>> {
    let folder = std::fs::read_dir(folder_name)?;

    let mut dict: HashMap<String, i32> = HashMap::new();
    let wc = word_count::WordCounter::new().unwrap();
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
                    .filter(|k| dict[*k] > low_threshold && dict[*k] < high_threshold)
                    .cloned()
                    .collect::<Vec<String>>();

    words
}


fn main() {
    let hmap = wc_all_files("./korpusz").unwrap();
    let words = cull_word_map(hmap, 99, 10099);

    let out = File::create("word_index.csv").unwrap();
    let mut writer = BufWriter::new(out);
    for (w, i) in words.iter().enumerate() {
        writer.write_all(format!("{}; {}\n", w, i).as_bytes()).unwrap();
    }
}
