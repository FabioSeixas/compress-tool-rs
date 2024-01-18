use crate::tree;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    os::unix::prelude::FileExt,
    str::SplitTerminator,
};

const OFFSET: usize = 255;
const OFFSET_HEADER: usize = 3;

struct HeaderBoundary {
    count: u8,
}

impl HeaderBoundary {
    fn new() -> Self {
        Self { count: 0 }
    }
    fn add(&mut self) {
        if self.count < 21 {
            self.count += 1;
        }
    }
    fn is_done(&self) -> bool {
        self.count == 21
    }
}

pub fn decompress_file(original_file_path: String) {
    let original_file =
        File::open(original_file_path).expect("Error while trying to open the original file");

    let code_table = read_header(original_file);
    println!("{:?}", code_table)
}

fn read_file_by_line(original_file_path: String) {
    let file =
        File::open(original_file_path).expect("Error while trying to open the original file");
    let mut code_table: tree::CodeTable = HashMap::new();
    for line in BufReader::new(file).lines() {

    }
}

fn read_header(file: File) -> (usize, tree::CodeTable) {
    let mut code_table: tree::CodeTable = HashMap::new();
    let mut buf: [u8; OFFSET_HEADER] = [0; OFFSET_HEADER];

    let mut offset: usize = 0;

    let mut header_content_ended = false;

    let mut header = HeaderBoundary::new();

    let mut current_item_char: Option<char> = None;

    let mut current_item_value: String = String::from("");

    while !header_content_ended {
        let mut parse_buffer_done = false;

        let amount_readed = file
            .read_at(&mut buf, offset.try_into().unwrap())
            .expect("error while trying to read the file");

        // if amount_readed < OFFSET {
        //     file_content_ended = true
        // }

        offset += OFFSET_HEADER;

        let mut buf_vec = buf.to_vec();

        let mut utf8_str = String::from("");

        while !parse_buffer_done {
            match String::from_utf8(buf_vec.clone()) {
                Ok(v) => {
                    parse_buffer_done = true;
                    utf8_str = v
                }
                Err(_) => {
                    println!("utf8 error {:?}", buf_vec);
                    buf_vec.pop();
                    offset -= 1;
                }
            };
        }

        println!("utf 8 string: {utf8_str}");

        for char in utf8_str.chars() {
            println!("char: {char}");

            if current_item_char == None && char == '\n' && header.is_done() {
                println!("END READ header");
                header_content_ended = true;
                break;
            }

            if current_item_char != None && char == ' ' {
                println!("jump");
                continue;
            }
            if current_item_char != None && char == ':' {
                println!("jump");
                continue;
            }
            if current_item_char == None && char == '\n' {
                println!("setting current item char: {char}");
                current_item_char = Some(char);
                continue;
            }

            if current_item_char != None && char == '\n' {
                println!(
                    "finish code, adding to table {}, {}",
                    current_item_char.unwrap(),
                    current_item_value
                );
                code_table.insert(current_item_char.unwrap(), current_item_value.clone());
                current_item_char = None;
                current_item_value = String::from("");
                continue;
            }
            if char == '=' {
                println!("header");
                header.add();
                continue;
            }

            if current_item_char == None {
                println!("setting current item char: {char}");
                current_item_char = Some(char);
                continue;
            }

            if let Some(current_char) = current_item_char {
                println!("current char: {current_char}");
                match char {
                    ':' => continue,
                    ' ' => continue,
                    '\n' => continue,
                    _ => {
                        println!("adding to current item value: {char}");
                        current_item_value.push(char);
                        continue;
                    }
                }
            }
        }

        // if offset > 25 {
        //     println!("current item char: {:?}", current_item_char);
        //     println!("current item value: {current_item_value}");
        //     println!("code table: {:?}", code_table);
        //     panic!()
        // }
    }

    (offset, code_table)
}
