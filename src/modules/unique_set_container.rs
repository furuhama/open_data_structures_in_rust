#[derive(Debug, PartialEq)]
pub struct Element {
    content: String,
}

impl Element {
    pub fn new(content: String) -> Self {
        Self {
            content: content,
        }
    }

    fn ref_content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug, PartialEq)]
pub struct USet {
    elements: Vec<Box<Element>>,
    size: usize,
}

// try to use trait
// define interface of USet struct
trait IsUSet {
    fn size(&self) -> usize;

    fn add(&mut self, Box<Element>) -> bool;

    fn remove(&mut self, String) -> Option<Box<Element>>;
}

impl USet {
    pub fn new() -> Self {
        Self {
            // use vector to manage elements
            elements: Vec::<Box<Element>>::new(),
            size: 0,
        }
    }
}

impl IsUSet for USet {
    fn size(&self) -> usize {
        self.size
    }

    fn add(&mut self, element: Box<Element>) -> bool {
        match self.elements.iter().find(|&e| *e == element) {
            // case: USet does not contain the new element -> success
            None => {
                self.elements.push(element);
                self.size += 1;
                true
            },
            // case: USet contains the new element -> failure
            Some(_) => {
                false
            }
        }
    }

    fn remove(&mut self, content: String) -> Option<Box<Element>> {
        // if USet does not contains the element with the given content,
        //     -> `?` operator returns None
        // else if USet contains the element,
        //     -> `?` operator unwraps Some(_) and returns index
        let index = self.elements.iter().position(|e| *e.content == content)?;
        self.size -= 1;
        Some(self.elements.remove(index))
    }
}

mod test {
    // to use USet methods defined in IsUSet trait,
    // you should use its trait explicitly
    use super::{Element, USet, IsUSet};

    #[test]
    fn basic() {
        let mut uset = USet::new();

        assert!(uset.add(Box::new(Element::new(String::from("one")))));
        assert_eq!(uset.size, 1);
        // this should fail, because USet already has `one`,
        assert!(!uset.add(Box::new(Element::new(String::from("one")))));
        // and the size should keep 1
        assert_eq!(uset.size, 1);

        assert!(uset.add(Box::new(Element::new(String::from("two")))));
        assert_eq!(uset.size, 2);

        assert_eq!(uset.remove(String::from("hogehogefugafuga")), None);
        assert_eq!(uset.size, 2);
        assert_eq!(uset.remove(String::from("two")), Some(Box::new(Element::new(String::from("two")))));
        assert_eq!(uset.size, 1);
        // this should fail, because `two` is already removed from USet,
        assert_eq!(uset.remove(String::from("two")), None);
        // and the size should keep 1
        assert_eq!(uset.size, 1);
    }
}
