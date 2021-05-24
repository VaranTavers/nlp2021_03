use std::path::Path;

mod settings;
mod word_count;
mod map_loader;
mod data_creator;
mod bag_of_docs;

fn main() -> Result<(), std::io::Error> {
    if !(Path::new("./word_index.csv").exists()) {
        println!("Recreating word_index.csv");
        word_count::create_word_index()?;
    }

    let dict = map_loader::parse_dict("./word_index.csv").unwrap();

    if !(Path::new("./training_data.csv").exists()) {
        println!("Recreating training_data.csv");
        data_creator::create_training_data("./korpusz", &dict, 5).unwrap();
    }

    if !(Path::new("./bag_of_docs.csv").exists()) {
        println!("Recreating bag_of_docs.csv");
        bag_of_docs::create_bag_of_docs("./korpusz", &dict, 5).unwrap();
    }

    Ok(())
}
