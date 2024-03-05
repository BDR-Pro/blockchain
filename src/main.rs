use serde_json::Value;
use std::fs;
use std::process::{Command, Stdio};
use std::time::Instant;

const NOTEBOOK_FILE: &str = "new_task.ipynb";
const TEMP_PYTHON_FILE: &str = "temp_cell.py";
const PYTHON_COMMAND: &str = "python";
const CODE_CELL_TYPE: &str = "code";
const TERAFLOP_INDICATOR: &str = "teraFLOP:";

fn main() {
    match mine() {
        Ok(()) => println!("Mining completed successfully."),
        Err(e) => eprintln!("Mining failed: {}", e),
    }
}

fn mine() -> Result<(), String> {
    println!("Mining notebook: {}", NOTEBOOK_FILE);
    let file_content = fs::read_to_string(NOTEBOOK_FILE)
        .map_err(|e| format!("Failed to read notebook file: {}", e))?;

    let notebook: Value = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let mut total_time = 0;
    let mut total_tera_flops = 0.0;

    if let Some(cells) = notebook["cells"].as_array() {
        for cell in cells {
            if cell["cell_type"] == CODE_CELL_TYPE {
                let source = cell["source"].as_str().unwrap_or_default();
                fs::write(TEMP_PYTHON_FILE, source)
                    .map_err(|e| format!("Failed to write Python file: {}", e))?;

                let (time, tera_flops) = execute_python_cell()?;
                total_time += time;
                total_tera_flops += tera_flops;
            }
        }
    }

    println!("Total time: {} seconds", total_time);
    println!("Total teraFLOPs: {}", total_tera_flops);
    Ok(())
}

fn execute_python_cell() -> Result<(u64, f64), String> {
    let start = Instant::now();
    let output = Command::new(PYTHON_COMMAND)
        .arg(TEMP_PYTHON_FILE)
        .stdout(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    let duration = start.elapsed().as_secs();
    let output_str = String::from_utf8_lossy(&output.stdout);

    let tera_flops = parse_tera_flops(&output_str)?;
    Ok((duration, tera_flops))
}

fn parse_tera_flops(output: &str) -> Result<f64, String> {
    output
        .split_whitespace()
        .collect::<Vec<&str>>()
        .windows(2)
        .find_map(|window| {
            if window[0] == TERAFLOP_INDICATOR {
                window[1].parse().ok()
            } else {
                None
            }
        })
        .ok_or_else(|| "teraFLOP value not found in output".to_string())
}
