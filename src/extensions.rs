pub trait StringExt {
    fn starts_with(&self, search: String, index: usize) -> bool;
}

impl StringExt for String {
    fn starts_with(&self, search: String, index: usize) -> bool {
        let start = &self[..index];
        println!("{}", start);

        return search == start;
    }
}