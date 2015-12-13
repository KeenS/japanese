extern crate encoding;
extern crate japanese;

use encoding::all::ISO_2022_JP;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use japanese::buf::DecodingReader;


fn main() {
    let file = File::open("src/japanese.txt").unwrap();
    let e = DecodingReader::new(ISO_2022_JP, &file);
    let mut br = BufReader::new(e);
    let b = br.fill_buf().unwrap();
    for line in b.lines() {
        println!("{}", line.unwrap());
    }
}
