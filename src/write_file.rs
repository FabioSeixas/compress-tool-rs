use crate::tree;
use std::{fs::File, io::Write, os::unix::prelude::FileExt};

const BASE_NUMBER: u8 = 2;

pub fn write_compressed_file(file_path: String, table: tree::CodeTable) -> std::io::Result<()> {
    let mut new_file_path = String::from(file_path.clone());
    new_file_path.push_str("_compressed");
    let mut new_file = File::create(&new_file_path).expect("Error while creating file");

    write_header(&mut new_file, &table).expect("Error while writing header");

    write_content(file_path, &mut new_file, table).expect("Error while writing content");

    Ok(())
}

fn write_header(file: &mut File, table: &tree::CodeTable) -> std::io::Result<()> {
    let mut buf = String::from("");

    // buf.push_str("=====================\n");

    for (char, encoded) in table.iter() {
        buf.push_str(&format!("{}: ", char));
        buf.push_str(&format!("{}\n", encoded));
    }

    buf.push_str("=====================\n");

    file.write_all(buf.as_bytes())
        .expect("Error while writing compressed file header");

    Ok(())
}

fn write_content(
    original_file_path: String,
    new_file: &mut File,
    table: tree::CodeTable,
) -> std::io::Result<()> {
    let original_file =
        File::open(original_file_path).expect("Error while trying to open the original file");

    let mut offset = 0;

    let mut file_content_ended = false;

    const OFFSET: usize = 255;

    let mut buf: [u8; OFFSET] = [0; OFFSET];

    while !file_content_ended {
        let mut parse_buffer_done = false;

        let amount_readed = original_file
            .read_at(&mut buf, offset)
            .expect("error while trying to read the file");

        if amount_readed < OFFSET {
            file_content_ended = true
        }

        offset += 255;

        let mut buf_vec = buf.to_vec();

        let mut utf8_str = String::from("");

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

        let mut current_bit_list: [u8; 8] = [0; 8];
        let mut current_bit_list_index: usize = 0;

        for char in utf8_str.chars() {
            let current_code = table.get(&char).unwrap();

            for bit_str in current_code.chars() {
                if let Ok(bit) = u8::from_str_radix(&String::from(bit_str), 2) {
                    current_bit_list[current_bit_list_index] = bit;
                    current_bit_list_index += 1;
                }

                if current_bit_list_index == 8 {
                    println!("current_bit_list: {:?}", current_bit_list);
                    let mut sum: u8 = 0;
                    let index_range: Vec<u8> = (0..=8).collect();
                    for (bit, index) in current_bit_list.iter().zip(index_range.iter()) {
                        let index_32: u32 = index
                            .clone()
                            .try_into()
                            .expect("Errro while converting u8 to u32");
                        let res = bit * BASE_NUMBER.pow(index_32);
                        println!("index: {:?}", index);
                        println!("bit: {:?}", bit);
                        println!("result: {:?}", res);
                        sum += res;
                        println!("sum: {:?}", sum);
                    }
                    new_file
                        .write_all(&[sum])
                        .expect("Error while writing compressed file");
                    current_bit_list_index = 0;
                }
            }
        }
    }

    Ok(())
}
