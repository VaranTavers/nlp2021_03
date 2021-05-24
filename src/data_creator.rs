use std::collections::HashMap;

use std::io::Write;
use std::io::BufWriter;
use std::fs::File;

use crate::settings::Settings;

fn parse_file(text: &str, dict: &HashMap<String, i32>, settings: &Settings) -> Vec<(String, String)> {
    let mut res = Vec::new();

    let text = text.to_lowercase();
    let text = settings.sep.replace_all(&text, " ");
    let text = settings.ignore.replace_all(&text, "");
    let sentences = text.split('.');
    for sentence in sentences {
        let words = sentence.split(' ').collect::<Vec<&str>>();
        let m = words.len();
        for (i, w) in words.iter().enumerate() {
            let min = (i as i32 - settings.window_size / 2).max(0) as usize;
            let max = (i as i32 + settings.window_size / 2).min(m as i32) as usize;
            for j in min..max {
                if j != i 
                    && dict.contains_key(&w.to_string()) 
                    && dict.contains_key(&words[j].to_string()) {
                    res.push((w.to_string(), words[j].to_string()));
                }
            }
        }
    }

    res
}


fn get_training_data(folder_name: &str, 
    dict: &HashMap<String, i32>, 
    window_size: i32) -> Result<Vec<(String, String)>, std::io::Error> {

    let mut res = Vec::new();
    let settings = Settings::new(window_size).unwrap();

    let folder = std::fs::read_dir(folder_name)?;

    for entry in folder {
        let text = std::fs::read_to_string(entry.unwrap().path())?;
        let mut v = parse_file(&text, dict, &settings);
        res.append(&mut v);
    }

    Ok(res)
}

pub fn create_training_data(folder_name: &str, 
    dict: &HashMap<String, i32>, 
    window_size: i32) -> Result<(), std::io::Error> {

    let v = get_training_data(folder_name, dict, window_size)?;

    let out = File::create("training_data.csv")?;
    let mut writer = BufWriter::new(out);
    for (a, b) in v {
        writer.write_all(format!("{},{}\n", a, b).as_bytes())?;
    }

    Ok(())
}