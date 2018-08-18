#[derive(Debug, PartialEq)]
pub struct Element {
    content: String,
}

impl Element {
    pub fn new(content: String) -> Element {
        Element {
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

trait IsUSet {
    fn size(&self) -> usize;

    fn add(&mut self, Box<Element>) -> bool;

    fn remove(&mut self, String) -> Option<Box<Element>>;
}

impl USet {
    pub fn new() -> USet {
        USet {
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
            None => {
                self.elements.push(element);
                self.size += 1;
                true
            },
            Some(_) => {
                false
            }
        }
    }

    fn remove(&mut self, content: String) -> Option<Box<Element>> {
        let index = self.elements.iter().position(|e| *e.content == content)?;
        self.size -= 1;
        Some(self.elements.remove(index))
    }
}

mod test {
    use super::{Element, USet, IsUSet};

    #[test]
    fn basic() {
        let mut uset = USet::new();

        assert!(uset.add(Box::new(Element::new(String::from("one")))));
        assert_eq!(uset.size, 1);
        assert!(!uset.add(Box::new(Element::new(String::from("one")))));
        assert_eq!(uset.size, 1);

        assert!(uset.add(Box::new(Element::new(String::from("two")))));
        assert_eq!(uset.size, 2);

        assert_eq!(uset.remove(String::from("hogehogefugafuga")), None);
        assert_eq!(uset.size, 2);
        assert_eq!(uset.remove(String::from("two")), Some(Box::new(Element::new(String::from("two")))));
        assert_eq!(uset.size, 1);
        assert_eq!(uset.remove(String::from("two")), None);
        assert_eq!(uset.size, 1);
    }
}
