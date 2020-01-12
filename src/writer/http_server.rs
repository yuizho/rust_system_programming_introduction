use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;

        handle_connection(stream)?;
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    // lossyは無効な関数が来た場合、無名文字として置き換える
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    stream.write(b"HTTP/1.1 200 OK\r\n\r\nHi!!")?;
    stream.flush()?;
    Ok(())
}