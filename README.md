# TermWhiz

**TermWhiz** is an AI-powered terminal assistant that explains shell commands in simple language before executing them. It leverages Google Gemini's generative language API to provide concise, human-friendly explanations, helping users understand what a command does before running it—making the terminal safer and more approachable for everyone.

---

## Features

- **AI-Powered Explanations:** Get clear, two-sentence explanations for any terminal command using Google Gemini.
- **Interactive Confirmation:** Review the explanation and confirm before executing any command.
- **Cross-Platform:** Works on Windows, macOS, and Linux.
- **Simple CLI & TUI:** Clean command-line interface with potential for terminal UI enhancements.

---

## How It Works

1. **Input a Command:**  
   Run `termwhiz <your-command>` or launch without arguments to enter a command interactively.
2. **AI Explanation:**  
   TermWhiz queries the Gemini API and displays a concise explanation of what the command does.
3. **User Confirmation:**  
   You decide whether to proceed with execution based on the explanation.
4. **Safe Execution:**  
   If confirmed, the command runs in your shell; otherwise, execution is cancelled.

---

## Installation

### Prerequisites

- **Rust** (edition 2021): [Install Rust](https://rustup.rs/)
- **Google Gemini API Key:**  
  - Get an API key from [Google AI Studio](https://aistudio.google.com/app/apikey).

### Clone and Build

```sh
git clone https://github.com/Kishan1835/termwiz.git
cd termwiz
cargo build --release
```

### Set Up Environment

Create a `.env` file in the project root:

```
API_KEY=your-gemini-api-key-here
```

---

## Usage

### Explain and Run a Command

```sh
./target/release/termwhiz "echo Hello from TermWhiz"
```

Or launch interactively:

```sh
./target/release/termwhiz
```

You’ll see:

```
🔤 Enter a command to explain: ls -la
🧠 Command: `ls -la`
📘 Explanation:
Lists all files and directories in the current directory, including hidden ones, in long format.
❓ Do you want to execute this? [y/N]:
```

---

## Global Installation

### 1. Build the Release Binary

From your project root, run:

```sh
cargo build --release
```

This will create the binary at `target/release/termwhiz.exe` (on Windows) or `target/release/termwhiz` (on Linux/macOS).

### 2. Add to Your PATH

#### **On Windows**

- Copy the binary to a directory already in your `PATH` (e.g., `C:\Windows\System32` or another directory you prefer), **or** add the `target\release` directory to your `PATH`:

**To add `target\release` to your PATH temporarily:**

```powershell
$env:PATH += ";C:\path\to\your\project\target\release"
```

**To add it permanently:**

1. Open the Start Menu and search for "Environment Variables".
2. Click "Edit the system environment variables".
3. In the System Properties window, click "Environment Variables".
4. Under "System variables", find and select the `Path` variable, then click "Edit".
5. Click "New" and add the full path to your `target\release` directory (e.g., `C:\Users\YourName\termwhiz\target\release`).
6. Click OK to save.

#### **On Linux/macOS**

- Copy the binary to `/usr/local/bin` (requires sudo):

```sh
sudo cp target/release/termwhiz /usr/local/bin/
```

Or add the `target/release` directory to your `PATH` in your shell profile (`.bashrc`, `.zshrc`, etc.):

```sh
export PATH="$PATH:/path/to/your/project/target/release"
```

### 3. Use TermWhiz Anywhere

Now you can run `termwhiz` from any terminal window:

```sh
termwhiz "ls -la"
```

or

```sh
termwhiz
```

**Tip:**  
If you update the binary, repeat the copy step to overwrite the old version in your global path.

---

## Configuration

- **API Key:**  
  Set the `API_KEY` in your `.env` file or as an environment variable.
- **Debug Mode:**  
  Run with `cargo run` in development to see debug logs.

---

## Dependencies

- [tokio](https://crates.io/crates/tokio) - Async runtime
- [reqwest](https://crates.io/crates/reqwest) - HTTP client
- [serde](https://crates.io/crates/serde), [serde_json](https://crates.io/crates/serde_json) - JSON parsing
- [dotenv](https://crates.io/crates/dotenv) - Environment variable management
- [crossterm](https://crates.io/crates/crossterm), [ratatui](https://crates.io/crates/ratatui) - Terminal UI (future enhancements)

---

## Contributing

Pull requests and issues are welcome! Please open an issue to discuss your ideas or report bugs.

---

## License

This project is licensed under the MIT License.

---

## Disclaimer

**Warning:** While TermWhiz helps explain commands, always review explanations and use caution before executing unfamiliar commands. 