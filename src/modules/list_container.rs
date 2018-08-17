use std::ptr::NonNull;

#[derive(Debug)]
pub struct Node {
    prev: Option<NonNull<Node>>,
    next: Option<NonNull<Node>>,
    content: String,
}

impl Node {
    pub fn new(content: String) -> Node {
        Node {
            prev: None,
            next: None,
            content: content,
        }
    }
}

#[derive(Debug)]
pub struct ListContainer {
    first: Option<NonNull<Node>>,
    last: Option<NonNull<Node>>,
}

impl ListContainer {
    pub fn new() -> ListContainer {
        ListContainer {
            first: None,
            last: None,
        }
    }

    pub fn push_first(&mut self, mut node: Box<Node>) {
        match self.first {
            // case: ListContainer is empty
            None => {
                let node_ptr = Box::into_raw_non_null(node);

                // set pointer for container
                self.first = Some(node_ptr);
                self.last= Some(node_ptr);
            },
            // case: ListContainer has some Nodes
            Some(_) => {
                unsafe {
                    // set pointer for pushed node
                    node.next = self.first;

                    let node_ptr = Box::into_raw_non_null(node);

                    // set pointer for previous first node
                    self.first.unwrap().as_mut().prev = Some(node_ptr);
                    // set pointer for container
                    self.first = Some(node_ptr);
                }
            },
        }
    }

    pub fn push_last(&mut self, mut node: Box<Node>) {
        match self.last {
            // case: ListContainer is empty
            None => {
                let node_ptr = Box::into_raw_non_null(node);

                // set pointer for container
                self.first = Some(node_ptr);
                self.last= Some(node_ptr);
            },
            // case: ListContainer has some Nodes
            Some(_) => {
                unsafe {
                    // set pointer for pushed node
                    node.prev = self.last;

                    let node_ptr = Box::into_raw_non_null(node);

                    // set pointer for previous last node
                    self.last.unwrap().as_mut().next = Some(node_ptr);
                    // set pointer for container
                    self.last = Some(node_ptr);
                }
            },
        }
    }

    pub fn pop_first(&mut self) -> Option<Box<Node>> {
        match self.first {
            // case: ListContainer is empty
            None => {
                None
            },
            // case: ListContainer has some Nodes
            Some(mut node_ptr) => {
                unsafe {
                    match node_ptr.as_ref().next {
                        // case: ListContainer has just one Node
                        None => {
                            // set pointer for container
                            self.first = None;
                            self.last = None;

                            Some(Box::from_raw(node_ptr.as_mut()))
                        },
                        // case: ListContainer has two or more Nodes
                        Some(next_ptr) => {
                            // set pointer for container
                            self.first = Some(next_ptr);
                            // set container for new first node
                            self.first.unwrap().as_mut().prev = None;

                            Some(Box::from_raw(node_ptr.as_mut()))
                        }
                    }
                }
            },
        }
    }

    pub fn pop_last(&mut self) -> Option<Box<Node>> {
        match self.last {
            // case: ListContainer is empty
            None => {
                None
            },
            // case: ListContainer has some Nodes
            Some(mut node_ptr) => {
                unsafe {
                    match node_ptr.as_ref().prev {
                        // case: ListContainer has just one Node
                        None => {
                            // set pointer for container
                            self.first = None;
                            self.last = None;

                            Some(Box::from_raw(node_ptr.as_mut()))
                        },
                        // case: ListContainer has two or more Nodes
                        Some(prev_ptr) => {
                            // set pointer for container
                            self.last = Some(prev_ptr);
                            // set pointer for new last node
                            self.last.unwrap().as_mut().next = None;

                            Some(Box::from_raw(node_ptr.as_mut()))
                        }
                    }
                }
            },
        }
    }
}

mod test {
    #[allow(unused)]
    use super::{ListContainer, Node};

    #[test]
    fn push_to_last_pop_from_last() {
        let mut list_container = ListContainer::new();

        list_container.push_last(Box::new(Node::new(String::from("one"))));
        list_container.push_last(Box::new(Node::new(String::from("two"))));
        list_container.push_last(Box::new(Node::new(String::from("three"))));

        // Container [Node("one"), Node("two"), Node("three")]

        assert_eq!(list_container.pop_last().unwrap().content, String::from("three"));
        assert_eq!(list_container.pop_last().unwrap().content, String::from("two"));
        assert_eq!(list_container.pop_last().unwrap().content, String::from("one"));
    }

    #[test]
    fn push_to_last_pop_from_first() {
        let mut list_container = ListContainer::new();

        list_container.push_last(Box::new(Node::new(String::from("one"))));
        list_container.push_last(Box::new(Node::new(String::from("two"))));
        list_container.push_last(Box::new(Node::new(String::from("three"))));

        // Container [Node("one"), Node("two"), Node("three")]

        assert_eq!(list_container.pop_first().unwrap().content, String::from("one"));
        assert_eq!(list_container.pop_first().unwrap().content, String::from("two"));
        assert_eq!(list_container.pop_first().unwrap().content, String::from("three"));
    }

    #[test]
    fn push_to_first_pop_from_last() {
        let mut list_container = ListContainer::new();

        list_container.push_first(Box::new(Node::new(String::from("one"))));
        list_container.push_first(Box::new(Node::new(String::from("two"))));
        list_container.push_first(Box::new(Node::new(String::from("three"))));

        // Container [Node("three"), Node("two"), Node("one")]

        assert_eq!(list_container.pop_last().unwrap().content, String::from("one"));
        assert_eq!(list_container.pop_last().unwrap().content, String::from("two"));
        assert_eq!(list_container.pop_last().unwrap().content, String::from("three"));
    }

    #[test]
    fn push_to_first_pop_from_first() {
        let mut list_container = ListContainer::new();

        list_container.push_first(Box::new(Node::new(String::from("one"))));
        list_container.push_first(Box::new(Node::new(String::from("two"))));
        list_container.push_first(Box::new(Node::new(String::from("three"))));

        // Container [Node("three"), Node("two"), Node("one")]

        assert_eq!(list_container.pop_first().unwrap().content, String::from("three"));
        assert_eq!(list_container.pop_first().unwrap().content, String::from("two"));
        assert_eq!(list_container.pop_first().unwrap().content, String::from("one"));
    }

    #[test]
    fn push_and_pop_randomly() {
        let mut list_container = ListContainer::new();

        list_container.push_first(Box::new(Node::new(String::from("one"))));
        list_container.push_first(Box::new(Node::new(String::from("two"))));
        list_container.push_last(Box::new(Node::new(String::from("three"))));
        list_container.push_last(Box::new(Node::new(String::from("four"))));

        // Container [Node("two"), Node("one"), Node("three"), Node("four")]

        assert_eq!(list_container.pop_first().unwrap().content, String::from("two"));
        assert_eq!(list_container.pop_last().unwrap().content, String::from("four"));
        assert_eq!(list_container.pop_first().unwrap().content, String::from("one"));
        assert_eq!(list_container.pop_last().unwrap().content, String::from("three"));
    }
}
