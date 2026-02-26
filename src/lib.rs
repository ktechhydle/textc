use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Index,
};

#[derive(Serialize, Deserialize, Debug)]
struct TextFile {
    dictionary: Vec<String>,
    contents: Vec<u16>,
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

    let mut dictionary: Vec<String> = Vec::new();
    let mut map: HashMap<String, u16> = HashMap::new();
    let mut contents: Vec<u16> = Vec::new();

    // build dictionary + hashmap
    for data in datas.iter() {
        let id = dictionary.len() as u16;
        dictionary.push(data.to_string());
        map.insert(data.to_string(), id);
    }

    // encode contents
    for data in split_text.iter() {
        if let Some(&id) = map.get(data) {
            contents.push(id);
        }
    }

    let text_file = TextFile {
        dictionary: dictionary,
        contents: contents,
    };

    let bytes = match bincode::serialize(&text_file) {
        Ok(j) => j,
        Err(e) => {
            println!("ERROR SERIALIZING DATA: {e}");

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
    let dictionary = text_file.dictionary;
    let contents = text_file.contents;

    let mut reconstructed = String::new();

    for &data in contents.iter() {
        let text = dictionary.index(data as usize);
        reconstructed.push_str(&text);
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
