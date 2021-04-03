use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn main() {
  let first_arg = env::args().nth(1);
  let fname = first_arg.expect("Please provide a filename as the first argument of the program");

  let mut handle = File::open(&fname).expect("Unable to open the file");
  let mut pos = 0;
  let mut buf = [0; BYTES_PER_LINE];

  while let Ok(_) = handle.read_exact(&mut buf) {
    print!("[0x{:08x}] ", pos);
    for byte in &buf {
      match *byte {
        0x00 => print!(". "),
        0xff => print!("## "),
        _ => print!("{:02x} ", byte),
      }
    }
    println!("");
    pos += BYTES_PER_LINE;
  }
}
