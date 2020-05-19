#[derive(Debug, PartialEq)]
pub struct Element {
    content: String,
}

impl Element {
    pub fn new(content: String) -> Self {
        Self { content: content }
    }

    fn ref_content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug, PartialEq)]
pub struct UniqSet {
    elements: Vec<Box<Element>>,
    size: usize,
}

// try to use trait
// define interface of UniqSet struct
trait IsUniqSet {
    fn size(&self) -> usize;

    fn add(&mut self, element: Box<Element>) -> bool;

    fn remove(&mut self, content: String) -> Option<Box<Element>>;
}

impl UniqSet {
    pub fn new() -> Self {
        Self {
            // use vector to manage elements
            elements: Vec::<Box<Element>>::new(),
            size: 0,
        }
    }
}

impl IsUniqSet for UniqSet {
    fn size(&self) -> usize {
        self.size
    }

    fn add(&mut self, element: Box<Element>) -> bool {
        match self.elements.iter().find(|&e| *e == element) {
            // case: UniqSet does not contain the new element -> success
            None => {
                self.elements.push(element);
                self.size += 1;
                true
            }
            // case: UniqSet contains the new element -> failure
            Some(_) => false,
        }
    }

    fn remove(&mut self, content: String) -> Option<Box<Element>> {
        // if UniqSet does not contains the element with the given content,
        //     -> `?` operator returns None
        // else if UniqSet contains the element,
        //     -> `?` operator unwraps Some(_) and returns index
        let index = self.elements.iter().position(|e| *e.content == content)?;
        self.size -= 1;
        Some(self.elements.remove(index))
    }
}

mod test {
    // to use UniqSet methods defined in IsUniqSet trait,
    // you should use its trait explicitly
    use super::{Element, IsUniqSet, UniqSet};

    #[test]
    fn basic() {
        let mut uset = UniqSet::new();

        assert!(uset.add(Box::new(Element::new(String::from("one")))));
        assert_eq!(uset.size, 1);
        // this should fail, because UniqSet already has `one`,
        assert!(!uset.add(Box::new(Element::new(String::from("one")))));
        // and the size should keep 1
        assert_eq!(uset.size, 1);

        assert!(uset.add(Box::new(Element::new(String::from("two")))));
        assert_eq!(uset.size, 2);

        assert_eq!(uset.remove(String::from("hogehogefugafuga")), None);
        assert_eq!(uset.size, 2);
        assert_eq!(
            uset.remove(String::from("two")),
            Some(Box::new(Element::new(String::from("two"))))
        );
        assert_eq!(uset.size, 1);
        // this should fail, because `two` is already removed from UniqSet,
        assert_eq!(uset.remove(String::from("two")), None);
        // and the size should keep 1
        assert_eq!(uset.size, 1);
    }
}
