use std::process::Command;

pub fn compile_shader(command: &str, args: &[&str]) -> String {
    match Command::new(command).args(args).output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).into(),
        Err(e) => format!("Error: {}", e),
    }
}