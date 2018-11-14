use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;

pub fn read(filename: &str) -> Vec<u8> {
  let mut f = OpenOptions::new().read(true).open(filename).unwrap();
  let mut buf = Vec::new();
  f.read_to_end(buf.as_mut()).unwrap();
  return buf;
}

pub fn write(filename: &str, data: &[u8]) {
  let mut f = OpenOptions::new().write(true).create(true).open(filename).unwrap();
  f.write(data).unwrap();
}
