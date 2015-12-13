extern crate encoding;
use self::encoding::{EncodingRef, DecoderTrap};
use std::io::Read;
use std::io;


pub struct DecodingReader<R> {
    inner: R,
    encoding: EncodingRef,
}

impl <R: Read> DecodingReader<R> {
    pub fn new(coding: EncodingRef, read: R)-> Self {
        DecodingReader {
            inner: read,
            encoding: coding
        }
    }
}

impl <R: Read> Read for DecodingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>{
        // TODO set appropriate buffer size
        let len = buf.len() / 2;
        let n = try!(self.inner.read(&mut buf[..len/2]));
        // TODO don't ignore input code
        let str = self.encoding.decode(&mut buf[..n], DecoderTrap::Ignore).unwrap();
        let mut i = 0;
        for (d, s) in buf.iter_mut().zip(str.bytes()) {
            *d = s;
            i+=1;
            if i == 0 {
                break
            };
        }
        assert!(i <= buf.len());
        Ok(i)
    }
}
