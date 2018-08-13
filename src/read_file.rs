use std::fs::File;
use std::io::{BufReader, BufRead};

struct Node<'a> {
    next: Option<Box<Node<'a>>>,
    content: &'a str,
}

impl<'a> Node<'a> {
    fn new(content: &'a str) -> Node<'a> {
        Node {
            next: None,
            content: content,
        }
    }

    fn set_next(&mut self, node: Node<'a>) {
        self.next = Some(Box::new(node));
    }
}

struct QueueContainer<'a> {
    first: Option<Box<Node<'a>>>,
    last: Option<Box<Node<'a>>>,
}

impl<'a> QueueContainer<'a> {
    fn new() -> QueueContainer<'a> {
        QueueContainer {
            first: None,
            last: None,
        }
    }

    fn append(&mut self, node: Box<Node<'a>>) {
        match self.first {
            None => {
                self.first = Some(node);
                self.last = None;
            },
            Some(_) => {
                let mut prev_last_node = self.last.unwrap();
                prev_last_node.next = Some(node);
                self.last = Some(node);
            },
        }
    }
}

pub fn read_file_and_write_from_bottom() {
    let f = File::open("seed_data.txt").expect("No such file or directory");
    let reader = BufReader::new(f);
    let lines = reader.lines();

    println!("{}", lines.last().unwrap().unwrap());
}
