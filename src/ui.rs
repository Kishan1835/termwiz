use std::io::{self, Write};

pub fn prompt_user(command: &str, explanation: &str) -> bool {
    println!("\n🧠 Command: `{}`", command);
    println!("\n📘 Explanation:\n{}\n", explanation);

    print!("❓ Do you want to execute this? [y/N]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("y")
}
