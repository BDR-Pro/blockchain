use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::StreamExt; // Ensure you have this for using 'next' and other stream combinators
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt; // Ensure you have this for using write_all and other AsyncWrite utilities
use blockchain_maker::count_files_in_folder;
use blockchain_maker::Blockchain;
use tokio::task;
use futures_util::SinkExt;
use tar;
pub mod nodes_contacting;


async fn ping_port_8888() -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws_stream, _) = connect_async("ws://localhost:8888").await?;

    // Send a ping message
    ws_stream.send(Message::Ping(vec![])).await?;

    // Wait for a pong message or ignore it, depending on your protocol
    if let Some(message) = ws_stream.next().await {
        match message? {
            Message::Pong(_) => println!("Received pong from port 8888"),
            _ => println!("Unexpected message type from port 8888"),
        }
    }

    Ok(())
}

async fn save_file(bin: Vec<u8>) -> Result<(), std::io::Error> {
    let mut file = TokioFile::create("/temp_blocks.tar.gz").await?;
    file.write_all(&bin).await?;
    unzip_file().await?;
    let result = create_blockchain("/temp_blocks".to_string());
    let return_result = if result {

        Result::Ok(())
    }
    else {
        Result::Err(std::io::Error::new(std::io::ErrorKind::Other, "Error creating blockchain"))
    };
    return_result
}

fn usize_to_i32(value: Result<usize, std::io::Error>) -> Result<i32, &'static str> {
    let value = value.unwrap();
    if value > i32::MAX as usize {
        Err("Value too large to fit into an i32")
    } else {
        Ok(value as i32)
    }
}

fn create_blockchain(path:String) -> bool {
    let local_path = path.clone();
    let blockchain: Blockchain = Blockchain::new(50,2^16);
    let result: bool = blockchain.validate_chain(local_path);
    // move not by renaming to /my_blockss
    if result == false {
        return false;
    }
    let the_path = path.clone();
    let last_block: i32 = usize_to_i32(count_files_in_folder(the_path)).unwrap();
    let first_block: i32 = usize_to_i32(count_files_in_folder("my_blocks")).unwrap();
    for i in first_block..last_block + 1 {
        let file_name = format!("/temp_blocks/block_{}.json", i);
        let new_file_name = format!("/my_blocks/block_{}.json", i);
        std::fs::rename(file_name, new_file_name).expect("Error moving file");
    }

    result
}

async fn unzip_file() -> Result<(), std::io::Error> {
    let path_to_unpack = "/temp_blocks".to_string();
    let tar_gz_path = "/temp_blocks.tar.gz".to_string();

    task::spawn_blocking(move || {
        let tar_gz = std::fs::File::open(tar_gz_path)?;
        let mut archive = tar::Archive::new(tar_gz);
        archive.unpack(path_to_unpack)?;
        Ok::<(), std::io::Error>(()) // Specify the type explicitly here
    })
    .await??;  // Double question mark for propagating potential errors correctly.

    Ok(())
}

pub async fn send_a_message(message:String,receiver:String,type_message:i16) -> String {
    // Assuming Tor is now installed and configured to listen on the default SOCKS5 port
    nodes_contacting::tor_proxy();
    let client = Client::builder().proxy(proxy).build()?;

    let (mut ws_stream, _) = connect_async(format!("ws://{receiver}:8080")).await?;
    
    // Send a text message

    if type_message == 1 {
        ws_stream.send(Message::Text(message.into())).await?;
        // Wait for a response
        if let Some(message) = ws_stream.next().await {
            match message? {
                Message::Text(text) => println!("Received text message: {}", text),
                _ => println!("Unexpected message type"),
            }
            text
        }
    }
    // Send a binary message
    else if type_message == 2 {
        ws_stream.send(Message::Binary(message.into())).await?;
        // Wait for a response
        if let Some(message) = ws_stream.next().await {
            match message? {
                Message::Text(text) => println!("Received text message: {}", text),
                _ => println!("Unexpected message type"),
            }
            text
        }

    }
    // Send a ping message
    else if type_message == 3 {
        ws_stream.send(Message::Ping(message.into())).await?;
        // Wait for a pong message
        if let Some(message) = ws_stream.next().await {
            match message? {
                Message::Pong(_) => println!("Received pong"),
                _ => println!("Unexpected message type"),
            }
            return "pong";
        }


    }
    else {
        println!("Invalid message type");
    }



    Ok(())
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Can't bind to address");
    println!("Listening on: ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");
    let (mut write, mut read) = ws_stream.split();
        // Send a ping to port 8888 each time a new connection is established
        if let Err(e) = ping_port_8888().await {
            eprintln!("Error pinging port 8888: {}", e);
        }

    // Initial message to the client
    write.send(Message::Text("Hello, World!".into())).await.expect("Error sending message");

    while let Some(message_result) = read.next().await {
        let message = match message_result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        };

        match message {
            Message::Text(text) => println!("Received text message: {}", text),
            Message::Binary(bin) => {
                // Assuming `save_file` is a proper async function you've implemented
                match save_file(bin).await { // Adjust path as needed
                    Ok(_) => println!("Received binary data and saved"),
                    Err(e) => eprintln!("Error saving binary data: {}", e),
                }
            },
            Message::Ping(data) => {
                // Respond with Pong
                if let Err(e) = write.send(Message::Pong(data)).await {
                    eprintln!("Error sending Pong: {}", e);
                }
            },
            Message::Close(_) => {
                println!("Received Close");
                // Here you might want to do something to cleanly close the connection
                break; // Exit the loop to end the connection handling
            },
            _ => () // Handle other message types or do nothing
        }
    }
}
