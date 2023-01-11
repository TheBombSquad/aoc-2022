use std::{rc::{Rc, Weak}, cell::RefCell};

#[derive(Clone, Debug)]
pub enum FileSystemElement {
    File(File),
    Directory(Rc<RefCell<Directory>>),
}

impl FileSystemElement {
    fn file(&self) -> Result<&File, ()> {
        if let FileSystemElement::File(f) = self { Ok(f) }
        else { Err(()) }
    }
    fn directory(&self) -> Result<Rc<RefCell<Directory>>, ()> {
        if let FileSystemElement::Directory(d) = self { Ok(d.clone()) }
        else { Err(()) }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: u32,
}

#[derive(Default, Clone, Debug)]
pub struct Directory {
    name: String,
    contents: Vec<FileSystemElement>,
    parent: Option<Weak<RefCell<Directory>>>,
}

impl Directory {
    // Add a new file to the directory
    fn add_file(&mut self, name: &str, size: u32) {
        let file = File { name: name.to_owned(), size };
        self.contents.push(FileSystemElement::File(file));
    }

    // Add a new directory to the directory
    fn add_directory(&mut self, name: &str, parent: Option<Rc<RefCell<Directory>>>) -> Rc<RefCell<Directory>> {
        let parent_ref = parent.map(|d| Rc::downgrade(&d));

        let directory = Rc::new(RefCell::new(Directory {
            name: name.to_owned(),
            parent: parent_ref,
            ..Default::default()
        }));

        let cloned_rc = directory.clone();
        self.contents.push(FileSystemElement::Directory(directory));
        cloned_rc
    }

    // Get a list of all directories in the directory
    fn get_child_directories(&self) -> Vec<Rc<RefCell<Directory>>> {
        self.contents
            .iter()
            .filter_map(|x| x.directory().ok())
            .collect()
    }

    //  Get a child directory by name. Assume the input is always good
    fn get_directory_by_name(&self, name: &str) -> Rc<RefCell<Directory>> {
        self.get_child_directories()
            .into_iter()
            .find(|x| x.borrow().name.eq(&name))
            .unwrap()
    }

    // Get a list of files in the directory
    fn get_files(&self) -> Vec<&File> {
        self.contents
            .iter()
            .filter_map(|x| x.file().ok())
            .collect()
    }

    // Get the total size of all of the files in this directory
    fn get_content_file_size(&self) -> u32 {
        self.get_files()
            .into_iter()
            .map(|x| x.size)
            .sum()
    }

    // Get the total size of the directory, including the size of any sub-directories 
    fn get_total_size(&self) -> u32 {
        // Get size of all files in current directory 
        let mut current_size = self.get_content_file_size();

        // Get children 
        let children = self.get_child_directories();

        // Return file size only if we have no child directories 
        if children.is_empty() {
            current_size
        }
        // Otherwise recurse into child directories and get their size
        else {
            for child in &children {
                current_size += child.borrow().get_total_size();
            }
            current_size
        }
    }

    // Call get_total_size on every child directory found, recursively
    fn get_all_total_sizes<'a>(&'a self, overall: &'a mut Vec<u32>) -> &'a mut Vec<u32> {
        let children = self.get_child_directories();
        if !children.is_empty(){
            for child in &children {
                child.borrow().get_all_total_sizes(overall);
            }
        }
        overall.push(self.get_total_size());
        overall
    }

    // Get the directory's parent directory, if it exists
    fn get_parent_directory(&self) -> Option<Rc<RefCell<Directory>>> {
        if let Some(p) = &self.parent {
            p.upgrade()
        }
        else {
            None
        }
    }
}

pub enum ReaderState {
    ReadCommandOutput,
    ReadCommandInput,
}

// A "fake" generator since cargo-aoc doesn't let you pass Rc<RefCell<T>> to solvers for some
// reason...
pub fn get_root_directory(input_lines: &[String]) -> Rc<RefCell<Directory>> {
    // A "true" root, because the first command involves changing directory to '/'
    let mut active_directory = Rc::new(RefCell::new(Directory::default()));

    // We want to keep this reference alive so the rest of the tree structure doesn't get dereffed
    // when we traverse nodes
    let root_dir = active_directory.borrow_mut(). add_directory("/", None);

    let mut state = ReaderState::ReadCommandInput;

    for line in input_lines {
            let command_args = line.split(' ').collect::<Vec<&str>>();
            if command_args.first().unwrap().eq(&"$") { state = ReaderState::ReadCommandInput };
            match state {
                ReaderState::ReadCommandInput => {
                    let (command, argument) = (command_args.get(1).unwrap(), command_args.get(2).unwrap_or(&""));
                    match *command {
                        "ls" => state = ReaderState::ReadCommandOutput,
                        "cd" => {
                            if argument.eq(&"..") { 
                                let active_borrow = active_directory.borrow().get_parent_directory().clone();
                                active_directory = active_borrow.unwrap();
                            }
                            else {
                                let new_directory = active_directory.borrow().get_directory_by_name(argument).clone();
                                active_directory = new_directory;
                            }
                        }
                        _ => (),
                    }
                },

                ReaderState::ReadCommandOutput => {
                    let (arg1, name) = (command_args.first().unwrap(), command_args.get(1).unwrap());
                    // A new file
                    if let Ok(size) = arg1.parse::<u32>() {
                        active_directory.borrow_mut().add_file(name, size);
                    }
                    // A new directory
                    else {
                        active_directory.borrow_mut().add_directory(name, Some(active_directory.clone()));
                    }
                }

            }
    }
    root_dir
}
#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<String>
{
    input.lines().map(|x| x.to_owned()).collect()
}

#[aoc(day7, part1)]
pub fn solver_part1(input: &[String]) -> u32 {
    let root = get_root_directory(input);
    let borrowed_root = root.borrow();
    let mut total_sizes_result = Vec::new();
    let total_sizes_vec = borrowed_root.get_all_total_sizes(&mut total_sizes_result);
    let result: u32 = total_sizes_vec.iter().copied().filter(|x| x <= &100_000).sum();
    result
}

#[aoc(day7, part2)]
pub fn solver_part2(input: &[String]) -> u32 {
    const TOTAL_SPACE: u32 = 70_000_000;
    const NEEDED_SPACE: u32 = 30_000_000;

    let root = get_root_directory(input);
    let borrowed_root = root.borrow();

    let current_free_space = TOTAL_SPACE - borrowed_root.get_total_size();
    let space_to_free = NEEDED_SPACE - current_free_space;

    println!("current free: {current_free_space:}, space to free: {space_to_free:}");
    let mut total_sizes_result = Vec::new();
    let total_sizes_vec = borrowed_root.get_all_total_sizes(&mut total_sizes_result);
    total_sizes_vec.sort_unstable(); 
    total_sizes_vec.iter().copied().find(|x| x >= &space_to_free).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_test() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let lines = input_generator(input);
        let base_root = get_root_directory(lines.as_slice());
        let root = base_root.borrow().get_directory_by_name("/");
        let borrowed_root = root.borrow();
        println!("{borrowed_root:#?}");
        println!("get_total_size: {:?}", borrowed_root.get_total_size());
        let mut size_vec = Vec::new();
        let all_total_sizes = borrowed_root.get_all_total_sizes(&mut size_vec);
        println!("computed total sizes: {all_total_sizes:?}");
        let filtered_and_summed: u32 = all_total_sizes.iter().copied().filter(|x| x <= &100_000).sum();
        println!("filtered result: {filtered_and_summed:?}");
        assert_eq!(filtered_and_summed, 95437);
    }
}
