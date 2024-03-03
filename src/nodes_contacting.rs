use std::fs;
use rand::Rng;
use reqwest;
use Tor_Traffic_Router::{is_tor_installed_unix, install_tor};
use zip::ZipArchive;
use blockchain::{count_files_in_folder, validate_chain,load_chain_from_disk};

pub fn tor_proxy(){
    let tor_installed =  is_tor_installed_unix();

    if !tor_installed {
        println!("Tor is not installed. Installing...");
        install_tor();
    } else {
        println!("Tor is already installed. Proceeding...");
        // Start Tor
        Command::new("tor").spawn()?;
        
    }

}

pub async fn download_blockchain() {
    // Read the contents of the file
    let file_contents = fs::read_to_string("/nodes/onion.txt")
        .expect("Failed to read the file");

    // Parse the lines and store them in a vector
    let lines: Vec<&str> = file_contents.lines().collect();

    // Generate a random index
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..lines.len());

    // Retrieve the line at the random index
    let mut random_line = lines[random_index];

    // Assuming Tor is now installed and configured to listen on the default SOCKS5 port
    let proxy = reqwest::Proxy::all("socks5://127.0.0.1:9050")?;
    let blockleng = count_files_in_folder("my_blocks");
    random_line = format!("{}/check?block_number={}", random_line, blockleng);
    let client = Client::builder().proxy(proxy).build()?;
    let response = client.get(random_line).send()?;
    let body = response.text()?;
    if body.contains("The blockchain is ahead of you please sync."){

    random_line = format!("{}/sync?block_number={}", random_line, blockleng);
    let response = client.get(random_line).send()?;
    // save file to my_blocks
    let bytes = response.bytes()?;

    // Save the ZIP file locally
    let zip_path = "downloaded_blocks.zip";
    let mut file = File::create(zip_path)?;
    file.write_all(&bytes)?;
    // Unzip the file
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract("temp_until_verifying")?;
    //temp until verifying is the folder that contains the new blocks to be verified
    //REMOVE THE ZIP FILE
    fs::remove_file(zip_path)?;
    //VERIFY THE BLOCKCHAIN
    if validate_chain("temp_until_verifying"){
        //MOVE THE BLOCKS TO MY_BLOCKS
        for entry in fs::read_dir("temp_until_verifying")? {
            let entry = entry?;
            let path = entry.path();
            fs::copy(path, "my_blocks")?;
        }
        //REMOVE THE TEMP FOLDER
        fs::remove_dir_all("temp_until_verifying")?;

    }
    }
    else{
        println!("The blockchain is up to date.");

    }

    }

    // Handle the response as needed
    // ...
