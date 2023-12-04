use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct File {
    _name: String,
    size: i32
}

#[derive(Debug)]
struct FileSystem {
    current_directory_stack: Vec<String>,
    root_directory: Directory,
}


#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    directories: HashMap<String, Box<Directory>>,
}

#[derive(Debug, Clone)]
struct DirectoryInfo {
    _name: String,
    size: i32
}

impl Directory {
    pub fn from(directory: &str) -> Self {
        Directory {
            name: directory.to_string(),
            files: vec![],
            directories: HashMap::new(),
        }
    }

    pub fn try_insert_directory(&mut self, key: &str, directory: Directory) {
        if !self.directories.contains_key(key) {
            self.directories.insert(key.to_string(), Box::new(directory));
        }
    }

    pub fn insert_file(&mut self, file_name: &str, size: i32) {
        self.files.push(File { _name: file_name.to_string(), size });
    }
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            current_directory_stack: vec![],
            root_directory: Directory::from("/"),
        }
    }

    pub fn add_or_ignore_directory(&mut self, directory: &str) {
        let mut directory_pointer: &mut Directory = &mut self.root_directory;

        for path in &self.current_directory_stack[1..] {
            directory_pointer = directory_pointer.directories.get_mut(path.as_str()).unwrap();
        }

        directory_pointer.try_insert_directory(directory, Directory::from(directory));
    }

    pub fn directories_info(root_directory: &Directory) -> Vec<DirectoryInfo> {
        let mut vec: Vec<DirectoryInfo> = Vec::new();

        FileSystem::directories_info_rec(&mut vec, root_directory);

        vec
    }

    fn directories_info_rec(vec: &mut Vec<DirectoryInfo>, directory: &Directory) -> DirectoryInfo {
        let file_sizes = directory.files.iter().fold(0i32, |mut sum, file| {
            sum += file.size;
            sum
        });

        let mut child_size_sum = 0;
        for child in directory.directories.values() {
            let child_info = FileSystem::directories_info_rec(vec, child);
            child_size_sum += child_info.size;
        }


        let dir_info = DirectoryInfo {
            _name: directory.name.to_string(),
            size: file_sizes + child_size_sum,
        };

        vec.push(dir_info.clone());

        dir_info
    }


    pub fn add_file(&mut self, file_name: &str, size: i32) {
        let mut directory_pointer: &mut Directory = &mut self.root_directory;

        for path in &self.current_directory_stack[1..] {
            directory_pointer = directory_pointer.directories.get_mut(path.as_str()).unwrap();
        }

        directory_pointer.insert_file(file_name, size);
    }

    pub fn change_directory(&mut self, directory: &str) {
        if directory == "/" {
            self.current_directory_stack.clear();
            self.current_directory_stack.push(directory.to_string());
            return;
        }

        if directory == ".." {
            self.current_directory_stack.pop();
            return;
        }

        self.current_directory_stack.push(directory.to_string());
    }
}

pub struct Day7;

impl crate::year2022::Day for Day7 {
    fn date(&self) -> (i32, i32) { (7, 2022) }

    fn run(&self) {
        // let input = fs::read_to_string("src/year_2022/day7/testing.txt").unwrap();
        let input = fs::read_to_string("src/year_2022/day7/input.txt").unwrap();

        let mut file_system: FileSystem = FileSystem::new();

        for line in input.split('\n') {
            if line.starts_with('$') {
                let split_vec = &line.split(' ').collect::<Vec<&str>>()[..];
                if let [_, _, argument] = split_vec { // cd command
                    let argument = argument.trim();
                    file_system.change_directory(argument);
                } else if let [_, _] = split_vec { // ls command

                }

                continue;
            }

            let line_split = line.split(' ').collect::<Vec<&str>>();

            if line_split[0] == "dir" { // directory
                file_system.add_or_ignore_directory(line_split[1].trim());
                continue;
            }

            if line_split[0].parse::<i32>().is_ok() { // file
                file_system.add_file(line_split[1].trim(), line_split[0].parse::<i32>().unwrap());
                continue;
            }
        }

        let directories: Vec<DirectoryInfo> = FileSystem::directories_info(&file_system.root_directory);

        let mut sum = 0;

        for directory in &directories {
            if directory.size <= 100000 {
                sum += directory.size;
            }
        }

        println!("Part one: {}", sum);

        // part 2
        let total_disk_space = 70000000;
        let target_unused_space = 30000000;

        let root = directories.last().unwrap();

        let unused_space = total_disk_space - root.size;
        let still_required_space = target_unused_space - unused_space;

        let mut candidates: Vec<&DirectoryInfo> = vec![];

        for directory in &directories {
            if directory.size >= still_required_space {
                candidates.push(directory);
            }
        }

        let result = candidates.iter().min_by_key(|can| can.size).unwrap();
        println!("Part two: {:?}", result);
    }
}