use std::io::prelude::*;
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let data = b"example\n";

    let mut pos = 0;
    // with buffering
    // https://doc.rust-lang.org/std/io/struct.BufWriter.html
    let mut file = BufWriter::new(File::create("test.txt")?);

    while pos < data.len() {
        let bytes_written = file.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}