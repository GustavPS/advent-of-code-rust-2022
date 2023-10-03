#[derive(Debug)]
pub struct FileObj {
    name: String,
    size: usize
}

impl FileObj {
    pub fn new(name: &str, size: usize) -> Self {
        FileObj {
            name: name.to_string(),
            size
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}