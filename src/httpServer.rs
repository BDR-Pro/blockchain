use Tor_Traffic_Router::{is_tor_installed_unix, install_tor,stop_tor , config_file};
#[macro_use] extern crate rocket;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use blockchain::{count_files_in_folder, get_block_hash_from_file};
use zip::ZipArchive;



pub fn start_tor() -> Result<(), Box<dyn Error>>{
  
    let tor_installed = if cfg!(target_os = "windows") {
        is_tor_installed_windows()
    } else {
        is_tor_installed_unix()
    };
    if !tor_installed {
        println!("Tor is not installed. Installing...");
        install_tor();
        let text = format!("HiddenServiceDir {}
        HiddenServicePort 80 127.0.0.1:{}",env::current_dir().unwrap().to_str().unwrap(), 8000);
        let _ = config_file("etc/tor/torrc", &text);
    } else {
        println!("Tor is already installed. Proceeding...");
        // Start Tor
        Command::new("tor").spawn()?;
        
    }


}



pub fn stop_tor() -> Result<(), Box<dyn Error>>{
    stop_tor();
    Ok(())
}

pub fn copy_hostname() -> Result<String, Box<dyn Error>>{
    let mut file = File::open("hostname")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the alpha centauri paying service! Use POST to send a payment."
}

#[post("/", data = "<payment>")]
fn create(payment: String) -> std::io::Result<String> {
    // Specify the directory where you want to save the payment files.
    // Ensure this directory exists and is writable.
    let dir = "./my_keys"; // Adjust the path according to your needs
    let file_name = format!("payment_{}.pem", uuid::Uuid::new_v4()); // Generate a unique file name.
    let file_path = Path::new(dir).join(file_name);

    // Create and write to the file
    let mut file = File::create(&file_path)?;
    file.write_all(payment.as_bytes())?;

    // Respond with the path to the created file
    Ok(file_path.to_str().unwrap_or("Invalid file path").to_string())
}


#[get("/sync?<block_number>&<blockleng>")]
async fn sync(block_number: usize) -> Result<ReaderStream<Cursor<Vec<u8>>>, rocket::response::Debug<std::io::Error>> {
    let mut cursor = Cursor::new(Vec::new());
    {
        //send him any file after the blocknumber 
        let mut blockleng = count_files_in_folder("my_blocks").unwrap();
        if  blockleng == block_number as usize {
            return "The blockchain is up to date.";

        }
        if blockleng > block_number as usize {
        block_number += 1;
        blockleng += 1;
        
        let mut zip = ZipWriter::new(cursor);

        for i in block_number..=blockleng {
            let file_path = Path::new("my_blocks").join(format!("{}.json", i));
            let mut file = match File::open(file_path).await {
                Ok(f) => f,
                Err(e) => {
                    println!("Failed to open file: {:?}", e);
                    continue; // Skip files that cannot be opened
                }
            };

            let mut buffer = Vec::new();
            match file.read_to_end(&mut buffer).await {
                Ok(_) => (),
                Err(e) => {
                    println!("Failed to read file: {:?}", e);
                    continue; // Skip files that cannot be read
                }
            }

            let file_name = format!("{}.json", i);
            zip.start_file(file_name, FileOptions::default().compression_method(CompressionMethod::Stored)).expect("Failed to add file to ZIP");
            zip.write_all(&buffer).expect("Failed to write file to ZIP");
        }

        zip.finish().expect("Failed to complete ZIP file");
        cursor.set_position(0); // Reset cursor to the beginning of the ZIP file
    }

    Ok(ReaderStream::new(cursor))
}

#[get("/check?block_number=<block_number>")]
fn check() -> &'static str {
    let blockleng = count_files_in_folder("my_blocks").unwrap();
    if  blockleng == block_number as usize {
        return "The blockchain is up to date.";

    }

    if blockleng > block_number as usize {
        return "The blockchain is ahead of you please sync.";
    }

    if blockleng < block_number as usize {
        return "My blockchain is behind you please wait for me to sync";
    }

}
 
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, create]);
    rocket::build().mount("/blockchain", routes![sync, check]);
}