use crate::tree;
use std::{collections::HashMap, fs::File, io::Read, os::unix::prelude::FileExt};

const OFFSET: usize = 20;
const OFFSET_HEADER: usize = 500;

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

    let (offset, mut code_table) = read_header(&original_file);

    let mut decoding_table: HashMap<String, char> = HashMap::new();
    for (k, v) in code_table.drain() {
        decoding_table.insert(v, k);
    }
    read_and_parse_content(&original_file, offset, decoding_table);
}

fn read_and_parse_content(file: &File, initial_offset: usize, code_table: tree::DecodingTable) {
    let mut offset: usize = initial_offset;

    let mut file_content_ended = false;

    let mut buf: [u8; OFFSET] = [0; OFFSET];

    let mut decoded_content = String::from("");

    let mut decodable_chunk = String::from("");

    while !file_content_ended {
        let amount_readed = file
            .read_at(&mut buf, offset.try_into().unwrap())
            .expect("Error while trying to read the file");

        offset += OFFSET;

        let mut end_buf_at_index: Option<usize> = None;

        if amount_readed < OFFSET {
            end_buf_at_index = Some(amount_readed);
            file_content_ended = true;
        }

        for (index, byte) in buf.bytes().enumerate() {
            if end_buf_at_index == Some(index) {
                break;
            }
            let actual_byte = byte.expect("Error while reading byte");

            let mut zero_quotient = false;
            let mut quotient = actual_byte;

            let mut encoded_chunk = String::from("");

            while !zero_quotient {
                let remainder = quotient % 2;
                quotient = quotient / 2;
                encoded_chunk.push_str(&format!("{remainder}"));
                if quotient == 0 {
                    zero_quotient = true;
                }
            }

            for char in encoded_chunk.chars() {
                decodable_chunk.push(char);

                match code_table.get(&decodable_chunk) {
                    None => continue,
                    Some(decoded_char) => {
                        decoded_content.push(decoded_char.clone());
                        decodable_chunk.clear();
                    }
                }
            }
        }
    }
}

fn read_header(file: &File) -> (usize, tree::CodeTable) {
    let mut code_table: tree::CodeTable = HashMap::new();
    let mut buf: [u8; OFFSET_HEADER] = [0; OFFSET_HEADER];

    let mut offset: usize = 0;

    let mut header_content_ended = false;

    let mut header = HeaderBoundary::new();

    let mut current_item_char: Option<char> = None;

    let mut current_item_value: String = String::from("");

    while !header_content_ended {
        let mut parse_buffer_done = false;

        file.read_at(&mut buf, offset.try_into().unwrap())
            .expect("error while trying to read the file");

        offset += OFFSET_HEADER;

        let mut utf8_str = String::from("");

        let mut buf_vec = buf.to_vec();

        while !parse_buffer_done {
            match String::from_utf8(buf_vec.clone()) {
                Ok(v) => {
                    parse_buffer_done = true;
                    utf8_str = v
                }
                Err(_) => {
                    buf_vec.pop();
                    offset -= 1;
                }
            };
        }

        for char in utf8_str.chars() {
            if current_item_char == None && char == '\n' && header.is_done() {
                header_content_ended = true;
                break;
            }

            if current_item_char != None && char == ' ' {
                continue;
            }
            if current_item_char != None && char == ':' {
                continue;
            }
            if current_item_char == None && char == '\n' {
                current_item_char = Some(char);
                continue;
            }

            if current_item_char != None && char == '\n' {
                code_table.insert(current_item_char.unwrap(), current_item_value.clone());
                current_item_char = None;
                current_item_value = String::from("");
                continue;
            }
            if char == '=' {
                header.add();
                continue;
            }

            if current_item_char == None {
                current_item_char = Some(char);
                continue;
            }

            if let Some(_) = current_item_char {
                match char {
                    ':' => continue,
                    ' ' => continue,
                    '\n' => continue,
                    _ => {
                        current_item_value.push(char);
                        continue;
                    }
                }
            }
        }
    }

    (offset, code_table)
}
