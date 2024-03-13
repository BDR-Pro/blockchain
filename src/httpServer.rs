use Tor_Traffic_Router::{is_tor_installed_unix, install_tor,stop_tor , config_file};
#[macro_use] extern crate rocket;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use blockchain::{count_files_in_folder, get_block_hash_from_file};
use zip::ZipArchive;
pub mod nodes_contacting;



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



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, create]);
    rocket::build().mount("/blockchain", routes![sync, check]);
}