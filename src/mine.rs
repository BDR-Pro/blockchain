use serde_json::Value;
use std::fs;
use std::process::{Command, Stdio};
use std::time::Instant;

fn mine() {
    let file_content = fs::read_to_string("new_task.ipynb").unwrap();
    let notebook: Value = serde_json::from_str(&file_content).unwrap();

    let mut total_time = 0;
    let mut tera_flop = 0.0; // Assuming teraFLOP is a floating point value

    if let Some(cells) = notebook["cells"].as_array() {
        for cell in cells {
            if cell["cell_type"] == "code" {
                let source = cell["source"].as_str().unwrap_or_default();

                // Assuming Python script calculates and prints the teraFLOP value
                fs::write("temp_cell.py", source).unwrap();

                let start = Instant::now();
                let output = Command::new("python")
                    .arg("temp_cell.py")
                    .stdout(Stdio::piped())
                    .output()
                    .expect("Failed to execute process");
                let duration = start.elapsed();
                
                total_time += duration.as_secs();

                // Capture and parse output for teraFLOP value
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.contains("teraFLOP") {
                    let parts: Vec<&str> = output_str.split_whitespace().collect();
                    for (index, &part) in parts.iter().enumerate() {
                        if part == "teraFLOP:" { // Assuming the output format is "teraFLOP: <value>"
                            tera_flop = parts.get(index + 1).unwrap_or(&"0").parse().unwrap_or(0.0);
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("Total time: {} seconds", total_time);
    println!("Total teraFLOPs: {}", tera_flop);
}
