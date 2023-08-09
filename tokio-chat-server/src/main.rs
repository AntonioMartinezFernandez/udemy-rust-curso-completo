// ! Remember to include the 'tokio' library in the 'Cargo.toml' file ;)
use std::net::SocketAddr;

use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

// Use the Tokio macro to create the Async Runtime in 'main'
#[tokio::main]
async fn main() {
    /***********************************************
     *
     *          MULTIUSER CHAT SERVER
     *        USING TOKIO ASYNC RUNTIME
     *
     ***********************************************/

    // Create TCP listener
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    // Create a Tokio 'channel', an special type of channel with multiple producers and consumers,
    // with defined type String and capacity for 10 values.
    // The receiver is not directly used (we obtain a new receiver creating a subscription to the sender).
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);

    // Create a loop to maintain several TCP socket connections simultaneously
    loop {
        // Start TCP listener
        let (mut socket, client_addr) = listener.accept().await.unwrap();

        // Clone the channel sender and create its receiver (with 'subscribe')
        let tx_clone = tx.clone();
        let mut rx_clone = tx_clone.subscribe();

        // Create a Tokio 'Task' (internal thread, not OS thread) to manage every client connection
        tokio::spawn(async move {
            // Split TCP reader and writer
            let (tcp_reader, mut tcp_writer) = socket.split();

            // Create buffer to read TCP stream
            let mut buffer = BufReader::new(tcp_reader);

            // Create string to allocate received data
            let mut line_of_received_data = String::new();

            loop {
                // Use the Tokio 'select' macro (similar to Promise.race() in Javascript)
                // to execute 'read' or 'write' case and discard the other one.
                tokio::select! {
                    // ### TCP Read case ###

                    // Read from TCP stream
                    _ = buffer.read_line(&mut line_of_received_data) => { // this is a Future (like a JS Promise)
                        // Send the data through the channel sender
                        tx_clone.send((line_of_received_data.clone(), client_addr)).unwrap();
                        // Remove content of the string with previous data
                        line_of_received_data.clear();
                    }

                    // ### TCP Write case ###

                    // Receive the data from the channel receiver
                    received_data_from_channel = rx_clone.recv()=>{ // this is the other Future (like a JS Promise)
                        // Unwrap message (type Result to String)
                        let received_data = received_data_from_channel.unwrap();

                        // Obtains message and client address
                        let message_from_channel = received_data.0;
                        let client_addr_from_channel = received_data.1;

                        // If client address is different to the sender address, sent the message
                        if client_addr != client_addr_from_channel{
                            // Write the data to the TCP stream
                            tcp_writer.write_all(message_from_channel.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
