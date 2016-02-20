use std::collections::HashMap;
use std::io::{Read, Result};

pub trait  Atbash<R: Read> {
    fn atbash(self) -> FunnyReader<R>;
}

impl<R: Read> Atbash<R> for R {
    fn atbash(self) -> FunnyReader<R> {
        FunnyReader::new(self)
    }
}

pub struct FunnyReader<R> {
    inner: R,
    bytemap: HashMap<u8, u8>,
}

impl<R: Read> FunnyReader<R> {
    pub fn new(inner: R) -> FunnyReader<R> {
        FunnyReader {
            inner: inner,
            bytemap: default_bytemap(),
        }
    }
}

impl<R: Read> Read for FunnyReader<R> {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
      let ret = self.inner.read(buf);

      for byte in buf.iter_mut() {
          *byte = *self.bytemap.get(&byte).unwrap_or(byte);
      }

      ret
  }
}

fn default_bytemap() -> HashMap<u8, u8> {
  build_bytemap(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "abcdefghijklmnopqrstuvwxyz"
  )
}

fn build_bytemap(upper: &str, lower: &str) -> HashMap<u8,u8> {
    let upper: Vec<_> = upper.chars().map(|c| c as u8).collect();
    let lower: Vec<_> = lower.chars().map(|c| c as u8).collect();

    let upper = upper.iter().zip(upper.iter().rev()).map(|(&a, &b)| (a,b));
    let lower = lower.iter().zip(lower.iter().rev()).map(|(&a, &b)| (a,b));

    upper.chain(lower).collect()
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read};
    use super::Atbash;

    #[test]
    fn funny_read_works() {
        let mut funny_reader = Cursor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ").atbash();
        let mut buf = String::new();

        funny_reader.read_to_string(&mut buf).ok();
        assert_eq!("ZYXWVUTSRQPONMLKJIHGFEDCBA", buf);
    }
}
