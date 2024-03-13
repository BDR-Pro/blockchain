use std::fs;
use rand::Rng;
use reqwest;
use Tor_Traffic_Router::{is_tor_installed_unix, install_tor,is_tor_installed_windows};
use zip::ZipArchive;
use blockchain::{count_files_in_folder, validate_chain,load_chain_from_disk};
pub mod ws;

pub fn tor_proxy() -> reqwest::Client{
    
    let tor_installed = if cfg!(target_os = "windows") {
        is_tor_installed_windows()
    } else {
        is_tor_installed_unix()
    };
    if !tor_installed {
        println!("Tor is not installed. Installing...");
        install_tor();
    } else {
        println!("Tor is already installed. Proceeding...");
        // Start Tor
        Command::new("tor").spawn()?;
        let proxy = reqwest::Proxy::all("socks5://127.0.0.1:9050")?;
        let client = Client::builder().proxy(proxy).build()?;
        
        return client;
    }

}

pub async fn download_blockchain() {
    // Read the contents of the file
    let proxy=tor_proxy();
    let file_contents = fs::read_to_string("/nodes/onion.txt")
        .expect("Failed to read the file");

    // Parse the lines and store them in a vector
    let lines: Vec<&str> = file_contents.lines().collect();

    // Generate a random index
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..lines.len());

    // Retrieve the line at the random index
    let mut random_node = lines[random_index];

    // Assuming Tor is now installed and configured to listen on the default SOCKS5 port
    let blockleng = count_files_in_folder("my_blocks");
    let my_message = format!("/check?block_number={}",blockleng);
    send_a_message(my_message,random_node,1);


    }

    // Handle the response as needed
    // ...
