use once_cell::sync::OnceCell;
use std::{cell::RefCell, collections::HashMap, fs::File, io::Read};

mod tree;

static mut CHARS_COUNT: OnceCell<CharsMap> = OnceCell::new();

#[derive(Debug)]
struct CharsMap {
    map: RefCell<HashMap<char, u32>>,
}

impl CharsMap {
    fn new() -> Self {
        Self {
            map: RefCell::new(HashMap::new()),
        }
    }
    pub fn start() {
        unsafe {
            CHARS_COUNT
                .set(CharsMap::new())
                .expect("Error while creating chars map");
        }
    }
    pub fn get_list() -> HashMap<char, u32> {
        let map = unsafe { CHARS_COUNT.get().expect("error while...").map.borrow() };
        map.clone()
    }

    pub fn add(c: char) {
        let map = unsafe { CHARS_COUNT.get_mut().expect("error while...").map.get_mut() };

        match map.get(&c) {
            None => {
                map.insert(c, 1);
            }
            Some(current_count) => {
                map.insert(c, current_count + 1);
            }
        }
    }
}

fn main() {
    match std::env::args().nth(1) {
        Some(file_path) => {
            println!("{file_path}");
            read_file(file_path);
        }
        None => {
            panic!("no file path");
        }
    };
}

fn read_file(fp: String) {
    let mut abc = File::open(fp).expect("error while trying to open the file");
    let mut buf = vec![];
    abc.read_to_end(&mut buf)
        .expect("error while trying to read the file");
    let content = String::from_utf8(buf).expect("error while decode from utf8");
    CharsMap::start();
    for char in content.chars() {
        if char == '\n' {
            continue;
        }
        CharsMap::add(char);
    }

    tree::build_tree(CharsMap::get_list())
}
