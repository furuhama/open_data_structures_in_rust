use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use super::super::modules::list_container::{Node, ListContainer};

// introduction 1
pub fn read_file_and_write_from_bottom() {
    // `seed_data.txt` contains 1,000,000 lines of texts.
    // let f = File::open("seed_data.txt").expect("No such file or directory");
    let f = File::open("small_seed_data.txt").expect("No such file or directory");
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut list_container = ListContainer::new();
    lines.for_each(|line| list_container.push_last(Box::new(Node::new(line.unwrap()))));

    print_list_container_from_last(&mut list_container)
    // print_list_container_from_last_longer(&mut list_container)
}

// this function uses pattern match and call itself recursively in `Some` pattern.
// it calls function each pattern matching, and stack frame could be too deep easily.
// (seed_data.txt has 1,000,000 lines, but the default maximum of stack frame is about 30,000)
// if you really want to print each line of 1,000,000 lines,
//     - you should NOT call function for each line.
//     - you should use for loop or something. -> print_list_container_from_last_longer()
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

// this function is improved version of print_list_container_from_last().
// as written in print_list_container_from_last() comment,
// it should use loop or something to print really large file.
fn print_list_container_from_last_longer(list_container: &mut ListContainer) {
    loop {
        let result = list_container.pop_last();
        match result {
            None => {
                println!("=== End ===");
                break;
            },
            Some(node) => {
                println!("{}", node.ref_content())
            },
        }
    }
}

// introduction 2
pub fn read_each_50_lines_and_write() {
    let f = File::open("small_seed_data.txt").expect("No such file or directory");
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut list_container = ListContainer::new();

    let mut idx = 0 as usize;

    while let Ok(i) = read_50_lines_and_write(&mut lines, &list_container, idx) {
        idx = i;
        println!("Take a break, I'm little bit tired.");
    };
}

fn read_50_lines_and_write(lines_iter: &mut Lines<BufReader<File>>, list_container: &ListContainer, mut index: usize) -> Result<usize, &'static str> {
    for _ in 0..50 {
        if let Some(node) = lines_iter.next() {
            println!("{}", node.unwrap());
            index += 1;
        } else {
            return Err("EOF");
        }
    }
    Ok(index)
}
