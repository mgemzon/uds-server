use std::io::{Write, Read};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a Unix socket path
    let socket_path = "/tmp/my_socket.sock";

    // Create a buffer to store messages
    let buffer =  Arc::new(Mutex::new(Vec::<String>::new()));

    // Start a background thread for compressing and saving messages to disk
    let buffer_clone = Arc::clone(&buffer);
    thread::spawn(move || {
        loop {
            // Sleep for some time
            thread::sleep(std::time::Duration::from_secs(5));

            // Lock the buffer and retrieve messages
            let messages = buffer_clone.lock().unwrap().clone();

            // Compress and save the messages to disk
            for message in messages {
                let compressed_message = compress_message(&message);
                save_to_disk(&compressed_message);
            }
        }
    });

    // Create a Unix Domain Socket (UDS) listener
    let listener = UnixListener::bind(socket_path)
                                                .expect("Failed to create socket listener");
    
    println!("Server started, waiting for connections...");

    // Accept client connections and handle messages
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let buffer_clone = Arc::clone(&buffer);
                thread::spawn(move || {
                    handle_client(stream, buffer_clone);
                });
            }
            Err(err) => {
                println!("Failed to accept client connection: {:?}", err);
            }
        }
    }
}

fn handle_client(mut stream: UnixStream, buffer: Arc<Mutex<Vec<String>>>) {
    // Read client message
    let mut message = String::new();
    stream.read_to_string(&mut message).expect("Failed to read from client");

    // Lock the buffer and push the message
    let mut buffer = buffer.lock().unwrap();
    buffer.push(message);
}

fn compress_message(message: &str) -> String {
    // Compress the message using xxxx compression algorithm.
    // e.g. 'flate2', 'lz4'.
    format!("{} [compressed]", message)
}

fn save_to_disk(compressed_message: &str) {
    println!("Message saved to disk: {}", compressed_message);
}
