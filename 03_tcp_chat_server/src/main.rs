use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

// we tell rust to use the macro for tokyo async main, which uses futures (non-known values yet)
#[tokio::main]
async fn main() {
    // tcp echo server
    let addr = "[::1]:8080";

    // returns a future which we wait and unwrap the result
    let listener = TcpListener::bind(addr).await.unwrap();

    // accept and address new connection
    let (mut socket, _addr) = listener.accept().await.unwrap();

    // basic data buffer read and write
    // loop {
    //     // data buffer
    //     let mut buffer = [0u8; 1024];
    //
    //     // read into the buffer
    //     let bytes_read = socket.read(&mut buffer).await.unwrap();
    //
    //     // writes all bytes from the input buffer to the output buffer
    //     socket.write_all(&buffer[..bytes_read]).await.unwrap();
    // }

    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }
        writer.write_all(line.as_bytes()).await.unwrap();
    }
}
