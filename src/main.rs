use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let transmitter = tx.clone();
        let mut receiver = transmitter.subscribe();

        tokio::spawn(async move {
            let (read_socket, mut write_socket) = socket.split();

            let mut reader = BufReader::new(read_socket);
            let mut line = String::new();

            loop {
                tokio::select! {
                        result = reader.read_line(&mut line) => {
                                        if result.unwrap() == 0 {
                                            break;
                                        }
                                        transmitter.send((line.clone(), addr)).unwrap();
                                        line.clear();
                        }
                        result = receiver.recv() => {
                                        let (msg, other_addr) = result.unwrap();
                                        if addr != other_addr {
                                            write_socket.write_all(msg.as_bytes()).await.unwrap();
                                        }
                        }
                }
            }
        });
    }
}
