mod explain;
mod shell;
mod ui;

use dotenv::dotenv;
use std::env;
use std::io::{self, Write};
use std::process::Command;
use tokio;

#[tokio::main]
async fn main() {
    // Step 1: Load environment variables
    dotenv().ok();
    if cfg!(debug_assertions) {
        println!("🔄 Loaded .env file");
    }

    // Step 2: Capture CLI args or prompt the user
    let args: Vec<String> = std::env::args().collect();
    let command = if args.len() < 2 {
        print!("🔤 Enter a command to explain: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    } else {
        args[1..].join(" ")
    };

    if cfg!(debug_assertions) {
        println!("🖥️ Command received: {}", command);
    }

    // Step 3: Load API key
    let api_key = env::var("API_KEY").unwrap_or_else(|_| {
        eprintln!("❌ API_KEY not found. Please set it in your environment.");
        std::process::exit(1);
    });

    // Step 4: Fetch explanation
    let explanation = match explain::get_explanation(&api_key, &command).await {
        Ok(e) => e,
        Err(err) => {
            eprintln!("❌ Gemini error: {}", err);
            "No explanation available.".to_string()
        }
    };

    // Step 5: Ask the user for confirmation
    let should_run = ui::prompt_user(&command, &explanation);

    // Step 6: Execute command if confirmed
    if should_run {
        println!("\n⚙️ Executing...\n");

        let mut child = if cfg!(windows) {
            Command::new("cmd")
                .args(["/C", &command])
                .spawn()
                .expect("Failed to execute command")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&command)
                .spawn()
                .expect("Failed to execute command")
        };

        let _ = child.wait(); // Wait for interactive command to complete
    } else {
        println!("❌ Execution cancelled.");
    }
}
