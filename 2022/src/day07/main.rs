use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static TOTAL_SIZE: i32 = 70000000;
static NEED_FREE: i32 = 30000000;

#[derive(Debug)]
pub struct FSObject {
    is_dir: bool,
    size: i32,
    children: HashSet<String>,
}

impl FSObject {
    pub fn dir() -> RefCell<Self> {
        RefCell::new(Self {
            is_dir: true,
            size: 0,
            children: HashSet::new(),
        })
    }

    pub fn file(size: i32) -> RefCell<Self> {
        RefCell::new(Self {
            is_dir: false,
            size: size,
            children: HashSet::new(),
        })
    }

    pub fn set_size(&mut self, size: i32) {
        self.size = size;
    }
}

#[derive(Debug)]
pub struct FileSystem {
    // https://stackoverflow.com/questions/47737084/how-can-i-simultaneously-iterate-over-a-rust-hashmap-and-modify-some-of-its-valu
    details: HashMap<String, RefCell<FSObject>>,
}

impl FileSystem {
    pub fn empty() -> Self {
        Self {
            details: HashMap::new(),
        }
    }

    fn update_parent<'a, 'b>(&mut self, name: &String, obj: RefCell<FSObject>) {
        self.details.entry(name.clone().to_owned()).or_insert(obj);
        if let Some(parent) = get_parent(name.as_str()) {
            let parent_entry = self
                .details
                .entry(parent.to_owned())
                .or_insert(FSObject::dir());
            parent_entry.get_mut().children.insert(name.clone());
        }
    }

    pub fn add_dir(&mut self, name: &String) {
        self.update_parent(name, FSObject::dir());
    }

    pub fn add_file(&mut self, name: &String, size: i32) {
        self.update_parent(name, FSObject::file(size));
    }

    pub fn calculate_sizes(&mut self) {
        let mut did_something = true;
        while did_something {
            let mut maybe_did_something = false;
            for name in self.details.keys() {
                let mut obj = self
                    .details
                    .get(name) // change #3
                    .expect("Child not found.")
                    .borrow_mut();
                if !obj.is_dir || obj.size > 0 {
                    continue;
                }
                maybe_did_something = true;
                let mut should_skip = false;

                let mut total = 0;
                for child_name in obj.children.iter() {
                    let child = self.details[child_name].borrow();
                    if child.size == 0 {
                        should_skip = true;
                        break;
                    }

                    total += child.size;
                }
                if should_skip {
                    continue;
                }
                obj.set_size(total);
            }
            did_something = maybe_did_something;
        }
    }

    pub fn get_smallest_greater_than(self, over_size: i32) -> i32 {
        let mut min = TOTAL_SIZE;
        for obj in self.details.values() {
            let obj = obj.borrow();
            if obj.is_dir && obj.size >= over_size {
                if obj.size < min {
                    min = obj.size;
                }
            }
        }
        return min;
    }

    pub fn total_size(&self) -> i32 {
        self.details
            .get("/")
            .expect("root dir should exist")
            .borrow()
            .size
    }
}

fn main() {
    // --snip--
    let file_path = "tmp/day07/input.txt";
    println!("In file {}", file_path);

    let mut fs = FileSystem::empty();
    let mut curr_dir = "/".to_owned();
    fs.add_dir(&curr_dir);

    let mut is_ls_output = false;

    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if let Some(dirname) = is_cd(&ip) {
                    if dirname == "/" {
                        continue;
                    } else if dirname == ".." {
                        if let Some(parent) = get_parent(curr_dir.as_str()) {
                            curr_dir = parent.to_owned();
                        }
                    } else {
                        curr_dir = path_join(&curr_dir, dirname);
                        fs.add_dir(&curr_dir);
                    }
                } else if is_ls(&ip) {
                    is_ls_output = true;
                } else if is_ls_output {
                    let (name, maybe_size) = parse_ls_line(&ip);
                    let new_name = path_join(&curr_dir, name);
                    if let Some(size) = maybe_size {
                        fs.add_file(&new_name, size);
                    } else {
                        fs.add_dir(&new_name);
                    }
                } else {
                    println!("Unexpected line? {}", ip)
                }
            }
        }
    }
    fs.calculate_sizes();
    let to_delete = NEED_FREE - (TOTAL_SIZE - fs.total_size());
    println!(
        "FS just over {}: {}",
        to_delete,
        fs.get_smallest_greater_than(to_delete)
    )
}

fn get_parent(filename: &str) -> Option<&str> {
    if let Some(parent_path) = Path::parent(Path::new(filename)) {
        if let Some(parent) = parent_path.to_str() {
            return Some(parent);
        }
    }
    return None;
}

fn path_join(parent: &String, basename: &str) -> String {
    if let Some(together) = Path::new(parent).join(Path::new(basename)).to_str() {
        return together.to_owned();
    }
    return "".to_owned();
}

fn is_ls(line: &str) -> bool {
    return line.eq("$ ls");
}

fn is_cd(line: &str) -> Option<&str> {
    if line.starts_with("$ cd") {
        if let Some(last) = line.split(" ").last() {
            return Some(last);
        }
    }
    return None;
}

fn parse_ls_line(line: &str) -> (&str, Option<i32>) {
    let parts: Vec<&str> = line.split(" ").collect();
    if parts[0] == "dir" {
        return (parts[1], None);
    }
    return (parts[1], Some(parts[0].parse::<i32>().unwrap()));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
