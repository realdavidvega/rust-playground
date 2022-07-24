use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

// we tell rust to use the macro for tokyo async main, which uses futures (non-known values yet)
#[tokio::main]
async fn main() {
    // tcp echo server
    // returns a future which we wait and unwrap the result
    let addr = "[::1]:8080";
    let listener = TcpListener::bind(addr).await.unwrap();

    // broadcast channel by tokio
    let (tx, _rx) = broadcast::channel(10);

    loop {
        // accept and address new connection
        let (mut socket, addr) = listener.accept().await.unwrap();

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

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // move the client handling to its independent task (async block)
        tokio::spawn(async move {
            // split the socket in reader and writer
            let (reader, mut writer) = socket.split();

            // reader buffer
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            // read lines on the loop
            loop {
                // wait on multiple concurrent branches, gathering data into result
                tokio::select! {
                    // read line and send it
                    result = reader.read_line(&mut line) => {
                         if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    // receive msg
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
