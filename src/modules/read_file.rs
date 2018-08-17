use std::fs::File;
use std::io::{BufReader, BufRead};
use super::list_container::{Node, ListContainer};

pub fn read_file_and_write_from_bottom() {
    // `seed_data.txt` contains 1,000,000 lines of texts.
    // let f = File::open("seed_data.txt").expect("No such file or directory");
    let f = File::open("small_seed_data.txt").expect("No such file or directory");
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut list_container = ListContainer::new();
    lines.for_each(|line| list_container.push_last(Box::new(Node::new(line.unwrap()))));

    print_list_container_from_last(&mut list_container)
}

fn print_list_container_from_last(link_container: &mut ListContainer) {
    match link_container.pop_last() {
        None => {
            println!("=== End ===");
        },
        Some(node) => {
            println!("{}", node.ref_content());
            print_list_container_from_last(link_container);
        }
    }
}
