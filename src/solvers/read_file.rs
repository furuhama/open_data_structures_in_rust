use super::super::modules::list_container::{ListContainer, Node};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_files_and_do_something() {
    // `seed_data.txt` contains 1,000,000 lines of texts.
    // `small_seed_data.txt' contains about 250 lines of texts.
    // let mut lines = open_file_to_lines("seed_data.txt").unwrap();
    let mut lines = open_file_to_lines("small_seed_data.txt").unwrap();

    let mut list_container = ListContainer::new();

    // introduction 1
    // read_from_bottom(&mut lines, &mut list_container);

    // introduction 2
    read_each_50_lines(&mut lines, &mut list_container);
}

// Shared function

fn open_file_to_lines(filename: &str) -> Result<Lines<BufReader<File>>, &'static str> {
    match File::open(filename) {
        Ok(f) => {
            let reader = BufReader::new(f);
            return Ok(reader.lines());
        }
        Err(_) => {
            return Err("No such file or directory");
        }
    }
}

// introduction 1.1.1

fn read_from_bottom(lines: &mut Lines<BufReader<File>>, list_container: &mut ListContainer) {
    lines.for_each(|line| list_container.push_last(Box::new(Node::new(line.unwrap()))));

    print_list_container_from_last(list_container)
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
        }
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
            }
            Some(node) => println!("{}", node.ref_content()),
        }
    }
}

// introduction 1.1.2

pub fn read_each_50_lines(lines: &mut Lines<BufReader<File>>, list_container: &mut ListContainer) {
    let mut idx = 0;

    while let Ok(i) = read_50_lines_and_write(lines, list_container, idx) {
        idx = i;
        println!("Take a break, I'm little bit tired.");
    }
}

fn read_50_lines_and_write(
    lines_iter: &mut Lines<BufReader<File>>,
    list_container: &mut ListContainer,
    mut index: usize,
) -> Result<usize, &'static str> {
    for _ in 0..50 {
        // use if let (almost the same as match expression)
        if let Some(line) = lines_iter.next() {
            list_container.push_last(Box::new(Node::new(line.unwrap())));
            index += 1;
        } else {
            break;
        }
    }

    let mut p_idx = 0;
    for _ in 0..50 {
        match list_container.pop_first() {
            Some(node) => {
                println!("{}", node.ref_content());
                p_idx += 1;
            }
            None => {
                break;
            }
        }
    }

    if p_idx == 50 {
        Ok(index)
    } else {
        Err("EOF")
    }
}
