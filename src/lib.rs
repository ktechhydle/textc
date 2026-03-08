use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Serialize, Deserialize, Debug)]
enum Contents {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
}

#[derive(Serialize, Deserialize, Debug)]
struct TextFile {
    sequence: String,
    dictionary: Vec<String>,
    contents: Contents,
}

fn find_most_common_sequence(text: &str, length: usize) -> Option<(String, usize)> {
    if length == 0 || length > text.len() {
        return None;
    }

    let mut counts: HashMap<&str, usize> = HashMap::new();
    let mut max_count = 0;
    let mut most_common_seq: Option<String> = None;

    for i in 0..=text.len() - length {
        let sequence = text.get(i..i + length)?;
        let count = counts.entry(sequence).and_modify(|c| *c += 1).or_insert(1);

        if *count > max_count {
            max_count = *count;
            most_common_seq = Some(sequence.to_string());
        }
    }

    most_common_seq.map(|seq| (seq, max_count))
}

fn tokenize(text: &str) -> (String, Vec<String>) {
    let longest_word = text
        .split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap();
    let seq = find_most_common_sequence(text, longest_word.len())
        .unwrap()
        .0;
    let tokens = text.split(&seq).map(|s| s.to_string()).collect();

    (seq, tokens)
}

// Compresses `text` into a binary Vec
fn compress(text: &str) -> Vec<u8> {
    let (sequence, split_text) = tokenize(&text);
    let mut datas = split_text.clone();
    let mut seen = HashSet::new();
    datas.retain(|x| seen.insert(x.to_owned()));

    let mut dictionary: Vec<String> = Vec::new();
    let mut map: HashMap<String, u32> = HashMap::new();

    // build dictionary + hashmap
    for data in datas.iter() {
        let id = dictionary.len() as u32;
        dictionary.push(data.to_string());
        map.insert(data.to_owned(), id);
    }

    let dict_len = dictionary.len();

    let contents = if dict_len <= u8::MAX as usize {
        Contents::U8(split_text.iter().map(|data| map[data] as u8).collect())
    } else if dict_len <= u16::MAX as usize {
        Contents::U16(split_text.iter().map(|data| map[data] as u16).collect())
    } else {
        Contents::U32(split_text.iter().map(|data| map[data]).collect())
    };

    let text_file = TextFile {
        sequence,
        dictionary,
        contents,
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
    let text_file: TextFile = bincode::deserialize(&data).expect("ERROR DESERIALIZING DATA");
    let sequence = text_file.sequence;
    let dictionary = text_file.dictionary;
    let contents = text_file.contents;

    let mut parts = Vec::new();

    match contents {
        Contents::U8(vec) => {
            for id in vec {
                parts.push(dictionary[id as usize].clone());
            }
        }
        Contents::U16(vec) => {
            for id in vec {
                parts.push(dictionary[id as usize].clone());
            }
        }
        Contents::U32(vec) => {
            for id in vec {
                parts.push(dictionary[id as usize].clone());
            }
        }
    }

    parts.join(&sequence)
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

    println!("Original Size: {} bytes", text.len());

    let data = compress(&text);

    println!("Compressed Size: {} bytes", data.len());

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

#[test]
fn test_data_accuracy() {
    read_and_compress("tests/input.txt", "tests/output");
    read_and_decompress("tests/output.tzp", "tests/og_input.txt");

    let starting_data = fs::read_to_string("tests/input.txt").unwrap();
    let og_data_from_compression = fs::read_to_string("tests/og_input.txt").unwrap();

    assert_eq!(&starting_data, &og_data_from_compression);
}
