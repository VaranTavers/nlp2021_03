use std::path::Path;

mod word_count;
mod map_loader;
mod data_creator;

fn main() -> Result<(), std::io::Error> {
    if !(Path::new("./word_index.csv").exists()) {
        println!("Recreating word_index.csv");
        word_count::create_word_index()?;
    }

    if !(Path::new("./training_data.csv").exists()) {
        println!("Recreating training_data.csv");
        let dict = map_loader::parse_dict("./word_index.csv")?;
        data_creator::create_training_data("./korpusz", &dict, 5)?;
    }

    Ok(())
}
