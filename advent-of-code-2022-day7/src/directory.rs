use crate::file_obj::FileObj;

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    files: Vec<FileObj>,
    subdirectories: Vec<Directory>
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            files: Vec::new(),
            subdirectories: Vec::new()
        }
    }

    pub fn add_file(&mut self, file: FileObj) {
        self.files.push(file);
    }

    pub fn add_subdirectory(&mut self, directory: Directory) {
        self.subdirectories.push(directory);
    }

    pub fn get_size(&self) -> usize {
        let size = self.files.iter().map(|f| f.get_size()).sum::<usize>();
        size + self.subdirectories.iter().map(|sub_dir| sub_dir.get_size()).sum::<usize>()
    }

    pub fn get_sub_directory_by_name(&mut self, name: &str) -> Option<&mut Directory> {
        self.subdirectories.iter_mut().find(|d| d.name == name)
    }

    pub fn get_subdirectories(&mut self) -> &mut Vec<Directory> {
        &mut self.subdirectories
    }

    pub fn get_all_folders(&self) -> Vec<&Directory> {
        let mut directories = Vec::new();

        for dir in &self.subdirectories {
            directories.push(dir);
            directories.extend(dir.get_all_folders());
        }
        directories
    }
}
