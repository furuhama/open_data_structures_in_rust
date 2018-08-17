use std::fs::File;
use std::io::{BufReader, BufRead};
use super::list_container::{Node, ListContainer};

pub fn read_file_and_write_from_bottom() {
    let f = File::open("seed_data.txt").expect("No such file or directory");
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut list_container = ListContainer::new();
    lines.for_each(|line| list_container.push_last(Box::new(Node::new(line.unwrap()))));

    let last_node = list_container.pop_last();
    println!("{:?}", last_node);
}
