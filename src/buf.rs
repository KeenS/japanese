extern crate encoding;
use self::encoding::{EncodingRef, DecoderTrap};
use std::io::Read;
use std::io;
use std::vec::Vec;


pub struct EncodingReader<R> {
    inner: R,
    decoding: EncodingRef,
}

impl <R: Read> EncodingReader<R> {
    pub fn new(coding: EncodingRef, read: R)-> Self {
        EncodingReader {
            inner: read,
            decoding: coding
        }
    }
}

impl <R: Read> Read for EncodingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>{
        use std::iter::repeat;
        // TODO: choose appropriate size
        let mut vec = repeat(0).take(buf.len()/2).collect::<Vec<u8>>();
        let mut b = &mut vec[..];
        let _ = try!(self.inner.read(&mut b));
        // TODO don't ignore input code
        let str = self.decoding.decode(b, DecoderTrap::Ignore).unwrap();
        let bytes = str.into_bytes();
        for (d, s) in buf.iter_mut().zip(bytes.iter()) {
            *d = *s;
        }
        Ok(b.len())
    }
}
