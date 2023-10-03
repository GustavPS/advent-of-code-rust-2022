use crate::directory::Directory;
use crate::file_obj::FileObj;

#[derive(Debug)]
pub struct FileSystem {
    root_directory: Directory,
    disk_space: usize
}

impl FileSystem {
    pub fn new(root: Directory, disk_space: usize) -> Self {
        FileSystem {
            root_directory: root,
            disk_space
        }
    }

    pub fn get_all_folders(&self) -> Vec<&Directory> {
        self.root_directory.get_all_folders()
    }

    pub fn unused_space(&self) -> usize {
        self.disk_space - self.root_directory.get_size()
    }

    pub fn add_file(&mut self, dir_path: &str, file_name: &str, size: usize) {
        let directory = self.find_directory(dir_path).unwrap();
        directory.add_file(FileObj::new(file_name, size));
    }

    fn find_directory(&mut self, path: &str) -> Option<&mut Directory> {
        let path_parts: Vec<&str> = path.split('/').collect();
        let mut current_dir = &mut self.root_directory;

        for part in path_parts {
            if part.is_empty() {
                continue;
            }
            let sub_dir = current_dir.get_sub_directory_by_name(part);

            match sub_dir {
                Some(sub_dir) => current_dir = sub_dir,
                None => return None
            };
        }
        Some(current_dir)
    }

    pub fn create_directory(&mut self, path: &str) {
        let path_parts: Vec<&str> = path.split('/').collect();
        let mut current_dir = &mut self.root_directory;

        for part in path_parts {
            if part.is_empty() {
                continue;
            }
            let mut found = false;
            let mut subdir_index = 0;
            for (index, subdir) in &mut current_dir.get_subdirectories().iter().enumerate() {
                if subdir.name == part {
                    subdir_index = index;
                    found = true;
                    break;
                }
            }

            if !found {
                let new_dir = Directory::new(part);
                current_dir.add_subdirectory(new_dir);
                subdir_index = current_dir.get_subdirectories().len() - 1;
            }
            current_dir = &mut current_dir.get_subdirectories()[subdir_index];
        }
    }
}
