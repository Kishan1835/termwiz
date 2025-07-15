use std::process::Command;

pub fn run(command: &str) -> Result<String, String> {
    let output = if cfg!(windows) {
        Command::new("cmd").args(["/C", command]).output()
    } else {
        Command::new("sh").arg("-c").arg(command).output()
    };

    match output {
        Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
