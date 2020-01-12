use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // https://doc.rust-lang.org/std/net/struct.TcpStream.html
    let mut stream = TcpStream::connect("ascii.jp:80")?;
    stream.write(b"GET / HTTP/1.0\r\nHost: ascii.jp\r\n\r\n");
    let mut buff = String::new();
    stream.read_to_string(&mut buff);
    println!("{}", buff);
    Ok(())
}