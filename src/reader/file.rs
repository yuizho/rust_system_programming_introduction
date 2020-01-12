use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() -> io::Result<()> {
    let mut f = File::open("test.txt")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer);
    println!("{}", buffer);

    // with buffered reader
    // https://doc.rust-lang.org/std/io/trait.BufRead.html
    let f = File::open("test.txt")?;
    let mut reader = BufReader::with_capacity(4, f);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}