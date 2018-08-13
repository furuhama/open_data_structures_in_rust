use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_file_and_write_from_bottom() {
    let f = File::open("seed_data.txt").expect("No such file or directory");
    let reader = BufReader::new(f);
    let lines = reader.lines();

    println!("{}", lines.last().unwrap().unwrap());
}
