use std::env::current_dir;
use std::process::Command;
fn get_current_dir() -> String {
    let current_dir = current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();
    current_dir.to_string()
}
fn run_in_terminal(command: &str, cwd: &str) {
    Command::new("cmd")
        .args(&["/C", command])
        .current_dir(cwd)
        .output()
        .expect("failed to execute process");
}
fn main() {
    let mut current_dir = get_current_dir();
    run_in_terminal("git ", &current_dir);
    current_dir[0] = 'D';
}
