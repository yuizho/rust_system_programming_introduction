use std::io;
use std::io::{Read, Write};

fn main() -> io::Result<()> {
    let mut bytes: &[u8] = b"example2\n";
    let mut buf = [0; 9];

    // rustの&[u8]はReadやWriteを実装しているので直接、read_exactなどが使える
    bytes.read_exact(&mut buf)?;

    io::stdout().write(&buf)?;
    Ok(())
}