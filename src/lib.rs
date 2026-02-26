use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Serialize, Deserialize, Debug)]
struct WordMap {
    data: HashMap<String, u16>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Contents {
    data: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TextFile {
    word_map: WordMap,
    contents: Contents,
}

// Compresses `text` into a binary Vec
fn compress(text: &str) -> Vec<u8> {
    let mut split_text: Vec<String> = Vec::new();
    let mut current = String::new();

    for c in text.chars() {
        match c {
            ' ' | '\n' => {
                if !current.is_empty() {
                    split_text.push(current.clone());
                    current.clear();
                }
                split_text.push(c.to_string());
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        split_text.push(current);
    }

    let mut datas = split_text.clone();
    let mut seen = HashSet::new();
    datas.retain(|x| seen.insert(x.to_owned()));

    let mut word_map = WordMap {
        data: HashMap::new(),
    };
    let mut contents = Contents { data: Vec::new() };

    // create a unique id for each split chunk of text
    for (i, data) in datas.iter().enumerate() {
        word_map.data.insert(data.to_string(), i as u16);
    }

    for data in split_text.iter() {
        let id = match word_map.data.get(data) {
            Some(&id) => id,
            None => 0,
        };
        contents.data.push(id);
    }

    let text_file = TextFile {
        word_map: word_map,
        contents: contents,
    };

    let bytes = match bincode::serialize(&text_file) {
        Ok(j) => j,
        Err(e) => {
            println!("ERROR CREATING JSON DATA: {e}");

            Vec::new()
        }
    };

    bytes
}

// Decompresses `data` into a String
fn decompress(data: Vec<u8>) -> String {
    let text_file: TextFile = match bincode::deserialize(&data) {
        Ok(j) => j,
        Err(e) => {
            println!("ERROR CREATING JSON DATA: {e}");

            return "".to_string();
        }
    };
    let word_map_reversed: HashMap<u16, String> = text_file
        .word_map
        .data
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect();
    let contents_data = text_file.contents.data;

    let mut reconstructed = String::new();

    for data in contents_data.iter() {
        let text = word_map_reversed.get(data).unwrap();
        reconstructed.push_str(text);
    }

    reconstructed
}

pub fn read_and_compress(file: &str, output_name: &str) {
    let mut output_name = output_name.to_owned();
    output_name.push_str(".tzp");

    let text = match fs::read_to_string(file) {
        Ok(t) => t,
        Err(e) => {
            println!("ERROR READING FILE: {e}");

            return;
        }
    };
    let data = compress(&text);

    match fs::write(output_name, data) {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR WRITING FILE: {e}");

            return;
        }
    }
}

pub fn read_and_decompress(file: &str, output_name: &str) {
    let data = match fs::read(file) {
        Ok(t) => t.to_owned(),
        Err(e) => {
            println!("ERROR READING FILE: {e}");

            return;
        }
    };
    let text = decompress(data);

    match fs::write(output_name, text) {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR WRITING FILE: {e}");

            return;
        }
    }
}
