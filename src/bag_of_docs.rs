use std::collections::HashMap;

use std::io::Write;
use std::io::BufWriter;
use std::fs::File;

use crate::settings::Settings;

fn parse_file(text: &str, dict: &HashMap<String, i32>, settings: &Settings) -> Vec<i32> {
    let mut res = vec![0; 10000];

    let text = text.to_lowercase();
    let text = settings.sep.replace_all(&text, " ");
    let text = settings.ignore.replace_all(&text, "");
    let sentences = text.split('.');
    for sentence in sentences {
        let words = sentence.split(' ').collect::<Vec<&str>>();
        for w in words.iter() {
            if dict.contains_key(&w.to_string()) {
                res[dict[&w.to_string()] as usize] = 1;
            }
        }
    }

    res
}


fn get_bag_of_docs(folder_name: &str, 
    dict: &HashMap<String, i32>, 
    window_size: i32) -> Result<Vec<Vec<i32>>, std::io::Error> {

    let mut res = Vec::new();
    let settings = Settings::new(window_size).unwrap();

    let folder = std::fs::read_dir(folder_name)?;

    for entry in folder {
        let text = std::fs::read_to_string(entry.unwrap().path())?;
        let v = parse_file(&text, dict, &settings);
        res.push(v);
    }

    Ok(res)
}

pub fn create_bag_of_docs(folder_name: &str, 
    dict: &HashMap<String, i32>, 
    window_size: i32) -> Result<(), std::io::Error> {

    let v = get_bag_of_docs(folder_name, dict, window_size)?;

    let out = File::create("bag_of_docs.csv")?;
    let mut writer = BufWriter::new(out);
    for a in v {
        for b in a {
            writer.write_all(format!("{};", b).as_bytes())?;
        }
        writer.write_all(b"\n")?;
    }

    Ok(())
}