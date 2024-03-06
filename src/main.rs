use serde_json::Value;
use sha2::digest::Output;
use std::fs;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;

const NOTEBOOK_FILE: &str = "new_task.ipynb"; // Replace with your notebook filename
const PYTHON_COMMAND: &str = "python"; // Or "python3" depending on your system
const CODE_CELL_TYPE: &str = "code";
const COMBINED_SCRIPT_FILE: &str = "task.py";
const OUTPUT_FILE: &str = "output.txt";

fn main() {
    /*
    match run_notebook() {
        Ok(()) => println!("Notebook processed and output saved successfully."),
        Err(e) => eprintln!("Failed to process notebook: {}", e),
    }
    
    let  _ = get_teraflop();
  
    match backup_tasks() {
        Ok(()) => println!("Backup tasks successfully."),
        Err(e) => eprintln!("Failed to backup tasks: {}", e),
    }
       */
}

fn backup_tasks() -> Result<(), Box<dyn std::error::Error>> {
    //move to the backup folder make the folder if it does not exist
    let notebook = fs::read("new_task.ipynb")?;
    fs::create_dir_all("backup")?;
    // uuid 
    let uuid = uuid::Uuid::new_v4();
    let backup_file = format!("backup/{}.ipynb", uuid);
    fs::write(backup_file, notebook)?;
    let output = fs::read("output.txt")?;
    let backup_output_file = format!("backup/{}.txt", uuid);
    fs::write(backup_output_file, output)?;

    Ok(())
}


fn get_teraflop() -> Result<f64, Box<dyn std::error::Error>> {
    let file = fs::read_to_string(OUTPUT_FILE)?;
    let lines: Vec<&str> = file.lines().collect();
    let mut teraflop: f64 = 0.0;

    // Ensure the directory exists
    fs::create_dir_all("my_flops")?;

    for line in lines {
        if let Some(prefix) = line.strip_prefix("teraFLOP:") {
            teraflop = prefix.trim().parse()?;
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("my_flops/teraFLOP.txt")?;
            writeln!(file, "{}", teraflop)?; // This should now work correctly
        }
    }

    Ok(teraflop)
}

fn run_notebook() -> Result<(), String> {
    let file_content = fs::read_to_string(NOTEBOOK_FILE)
        .map_err(|e| format!("Failed to read notebook file: {}", e))?;

    let notebook: Value = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let mut combined_code = String::new();

    if let Some(cells) = notebook["cells"].as_array() {
        for cell in cells {
            if cell["cell_type"] == CODE_CELL_TYPE {
                if let Some(source) = cell["source"].as_array() {
                    for line in source {
                        if let Some(code_line) = line.as_str() {
                            combined_code.push_str(code_line);
                        }
                        combined_code.push('\n');
                    }
                }
            }
        }
    } else {
        return Err("No cells found in notebook".to_string());
    }

    // Save the combined Python code to a file
    fs::write(COMBINED_SCRIPT_FILE, &combined_code)
        .map_err(|e| format!("Failed to write combined Python script: {}", e))?;

    // Execute the combined Python script and capture its output
    let output = Command::new(PYTHON_COMMAND)
        .arg(COMBINED_SCRIPT_FILE)
        .output()
        .map_err(|e| format!("Failed to execute combined Python script: {}", e))?;

    // Write the output (stdout and stderr) to the output file
    fs::write(
        OUTPUT_FILE,
        format!(
            "stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    )
    .map_err(|e| format!("Failed to write output to file: {}", e))?;

    Ok(())
}
