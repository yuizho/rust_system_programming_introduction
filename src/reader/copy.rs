use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let mut reader = File::open("test.txt")?;
    let mut writer: Vec<u8> = vec![];

    io::copy(&mut reader, &mut writer)?;
    println!("{}", String::from_utf8_lossy(writer.as_ref()));
    Ok(())
}