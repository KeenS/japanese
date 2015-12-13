extern crate encoding;
extern crate japanese;

use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_2022_JP;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::from_utf8;
use japanese::buf::EncodingReader;


fn main() {
    let file = File::open("src/japanese.txt").unwrap();
    let e = EncodingReader::new(ISO_2022_JP, file);
    let mut br = BufReader::new(e);
    let b = br.fill_buf().unwrap();
    for line in b.lines() {
        println!("{}", line.unwrap());
    }
//    let utf8 = ISO_2022_JP.decode(b, DecoderTrap::Ignore);
//    let str = utf8.unwrap();
//    println!("{:?}", str);
//    println!("{}", str);
}
