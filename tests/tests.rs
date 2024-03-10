use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::protocol::Message;
use ws::{handle_connection, ping_port_8888, save_file}; 

#[tokio::test]
async fn test_handle_connection() {
    // Mock setup: Create a local server to accept one connection and then close
    let server = TcpListener::bind("127.0.0.1:0").await.unwrap(); // bind to a random free port
    let addr = server.local_addr().unwrap();

    tokio::spawn(async move {
        if let Ok((stream, _)) = server.accept().await {
            handle_connection(stream).await;
        }
    });

    // Connect to the mock server as a client
    let (ws_stream, _) = tokio_tungstenite::connect_async(format!("ws://{}", addr))
        .await
        .expect("Failed to connect");
    assert!(ws_stream.get_ref().peer_addr().is_ok());
    // Here you would continue to send/receive messages to test the `handle_connection` behavior.
}

#[tokio::test]
async fn test_ping_port_8888() {
    // This requires a WebSocket server running on port 8888 that echoes back pings
    // For actual tests, you may want to mock this server or ensure it is running
    let result = ping_port_8888().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_save_file() {
    // This test should verify that binary data can be saved to a file correctly
    // Ensure this points to a test directory that can be safely written to and cleaned up
    // change tar.gz to bin 
    // Open the .tar.gz file
    let bin = "test/test.tar.gz"; 
    let mut file = File::open(bin).await?;
    let mut buffer = Vec::new();

    // Read the file into the buffer
    let binary = file.read_to_end(&mut buffer).await?;

    let result = save_file(binary).await;
    assert!(result.is_ok());

    // Further checks can ensure the file exists and contains the correct data
}
