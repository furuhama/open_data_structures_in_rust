pub struct Element {
    content: String,
}

impl Element {
    pub fn new(content: String) -> Element {
        Element {
            content: content,
        }
    }

    pub fn ref_content(&self) -> &str {
        &self.content
    }
}

pub struct USet {
    elements: Vec<Box<Element>>,
}

trait IsUSet {
    fn size() -> usize;

    fn add(Box<Element>) -> bool;

    fn remove(String) -> Option<Box<Element>>;
}

impl USet {
    pub fn new() -> USet {
        USet {
            elements: Vec::<Box<Element>>::new()
        }
    }
}
