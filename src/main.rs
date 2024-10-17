use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    // Basic command execution
    let status = Command::new("ls").arg("-l").status()?;

    println!("Command finished with status: {}", status);

    // Capture command output
    let output = Command::new("echo")
        .arg("Hello from command line!")
        .output()?;

    println!(
        "Command output: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    // Run a command and get real-time output
    let mut child = Command::new("ping")
        .arg("google.com")
        .arg("-c")
        .arg("4")
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("Real-time output: {}", line?);
        }
    }

    // Run multiple commands in a pipeline
    let ls_child = Command::new("ls")
        .arg("-l")
        .stdout(Stdio::piped())
        .spawn()?;

    let grep_child = Command::new("grep")
        .arg("rust")
        .stdin(Stdio::from(ls_child.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()?;

    let output = grep_child.wait_with_output()?;
    println!(
        "Pipeline output: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    // Run a shell command with multiple arguments
    let complex_command = Command::new("sh")
        .arg("-c")
        .arg("echo 'Current directory:' && pwd && echo 'Files:' && ls -la")
        .output()?;

    println!(
        "Complex command output: {}",
        String::from_utf8_lossy(&complex_command.stdout)
    );

    // Handle command errors
    let result = Command::new("nonexistent_command").spawn();

    match result {
        Ok(_) => println!("Command succeeded"),
        Err(e) => println!("Command failed: {}", e),
    }

    // Run a command with environment variables
    let env_command = Command::new("sh")
        .arg("-c")
        .arg("echo $CUSTOM_VAR")
        .env("CUSTOM_VAR", "Hello from Rust!")
        .output()?;

    println!(
        "Environment variable output: {}",
        String::from_utf8_lossy(&env_command.stdout)
    );

    // Run a command in a specific directory
    let dir_command = Command::new("ls").current_dir("/tmp").output()?;

    println!(
        "Directory listing: {}",
        String::from_utf8_lossy(&dir_command.stdout)
    );

    // Using the helper function
    let files = run_command("ls", &["-l"])?;
    println!("Files: {}", files);

    // Using the shell command helper
    let complex_result = run_shell_command("ls -l | grep rust | wc -l")?;
    println!("Number of rust-related files: {}", complex_result);

    Ok(())
}

// Helper function to run a command and return its output as a String
fn run_command(command: &str, args: &[&str]) -> io::Result<String> {
    let output = Command::new(command).args(args).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Command failed with error: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}

// Helper function to run a shell command with pipes and redirections
fn run_shell_command(command: &str) -> io::Result<String> {
    let output = Command::new("sh").arg("-c").arg(command).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Shell command failed with error: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}
