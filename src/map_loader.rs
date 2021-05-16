use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub fn parse_dict(filename: &str) -> Result<HashMap<String, i32>, std::io::Error> {
    let mut res = HashMap::new();

    let file = File::open(filename)?;
    let file = BufReader::new(file);

    for line_op in file.lines() {
        let line = line_op?;
        let sp = line.split(';').collect::<Vec<&str>>();
        res.insert(sp[0].to_string(), parse_input!(sp[1], i32));
    }

    Ok(res)
}