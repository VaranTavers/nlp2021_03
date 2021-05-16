use std::collections::HashMap;

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


fn main() {
    println!("Hello, world!");
}
