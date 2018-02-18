#![feature(entry_and_modify)]
use std::collections::BTreeMap;

use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::fs::File;
use std::error::Error;
use std::path::Path;

pub fn x_or(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    for i in 0..a.len() {
        r.push(a[i] ^ b[i%b.len()])
    }
    r
}

fn open_file(fname: &str) -> Result<BufReader<File>, std::io::Error> {
    let path = Path::new(fname);
    let display = path.display();

    match File::open(&path) {
        Err(why) => {
            println!("couldn't open {}: {}", display, Error::description(&why));
            Err(why)
        },
        Ok(file) => Ok(BufReader::new(file)),
    }
}

fn build_corpus(text: &String, score :&mut BTreeMap<char, f64>) {

    for c in text.chars() {
        score.entry(c).and_modify(|e| { *e += 1.0 }).or_insert(1.0);
    }

    let total = text.len() as f64;
    for (_, v) in score.iter_mut() {
        *v /= total;
    }
}

pub fn build_corpus_from_file(fname: &str, score :&mut BTreeMap<char, f64>) {
    let mut file = open_file(fname).unwrap();
    let mut file_buf = String::new();

    file.read_to_string(&mut file_buf).unwrap();
    build_corpus(&file_buf, score)
}

fn score_text(text: &Vec<u8>, corpus: &BTreeMap<char, f64>) -> f64 {
    let mut score = 0.0;

    for c in text { 
        match corpus.get(&(*c as char)) {
            Some(c_score) => score += c_score,
            None => continue,
        }
    }

    score
}

pub fn find_single_key_x_or(text: Vec<u8>, corpus: &BTreeMap<char, f64>) -> (Vec<u8>, char, f64) {
    let mut score: f64 = 0.0;
    let mut res = Vec::new();
    let mut key: u8 = 0;

    for k in 0u8..255 {
        let k_vec = vec!(k);
        let out = x_or(&text, &k_vec);
        let s = score_text(&out, &corpus);
        if s > score {
            score = s;
            key = k;
            res = out.clone();
        }
    }

    (res, key as char, score)
}

pub fn read_split_lines(fname :&str) -> Vec<String> {
    let file = open_file(fname).unwrap();
    let res :Result<Vec<String>, std::io::Error> = file.lines().collect();

    match res {
        Ok(v) => return v,
        Err(e) => {
            println!("{}", e);
            panic!("Failed to parse file:")
            },
    } 
}