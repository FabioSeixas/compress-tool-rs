use once_cell::sync::OnceCell;
use std::{cell::RefCell, collections::HashMap, fmt::format, fs::File, io::Read};

mod decompress;
mod tree;
mod write_file;

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
    let file_path = std::env::args()
        .nth(2)
        .expect("Error while reading file path from args");
    match std::env::args().nth(1) {
        Some(command) => {
            println!("option {command} used:");
            match command
                .strip_prefix('-')
                .expect("Error while parsing option flag")
            {
                "c" => {
                    compress_file(file_path);
                }
                "d" => decompress::decompress_file(file_path),
                v => {
                    let msg = format!("Flag '{v}' not supported");
                    panic!("{msg}");
                }
            }
        }
        None => {
            panic!("no file path");
        }
    };
}

fn compress_file(file_path: String) {
    let mut abc = File::open(file_path.clone()).expect("error while trying to open the file");
    let mut buf = vec![];
    abc.read_to_end(&mut buf)
        .expect("error while trying to read the file");
    let content = String::from_utf8(buf).expect("error while decode from utf8");
    CharsMap::start();
    for char in content.chars() {
        // if char == '\n' {
        //     continue;
        // }
        CharsMap::add(char);
    }

    let code_table = tree::build_tree(CharsMap::get_list());
    write_file::write_compressed_file(file_path, code_table)
        .expect("Error while writing compressed file");
}
