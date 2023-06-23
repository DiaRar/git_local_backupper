use std::env::{current_dir, args};
use std::path::Path;
use std::process::{Command, Output};
fn get_current_dir() -> String {
    let current_dir = current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();
    current_dir.to_string()
}
fn run_in_terminal(command: &str, cwd: &str) -> Output {
    Command::new("cmd")
        .args(&["/C", command])
        .current_dir(cwd)
        .output()
        .expect("IO Error")

}

fn main() {
    let current_dir = get_current_dir();
    let mut destination_dir = current_dir.clone();
    destination_dir.replace_range(0..1, "D");
    match Path::new(&destination_dir).exists() {
        true => println!("Destination directory exists"),
        false => {
            run_in_terminal(&format!("mkdir {}", &destination_dir), &current_dir);
        }
    }
    let commit_message = args().nth(1).unwrap_or("Empty".to_string());
    let adding_dir = run_in_terminal(&format!("git remote add local {}", &destination_dir), &current_dir);
    match adding_dir.status.success() {
        true => println!("Remote added"),
        false => println!("Remote already exists - {}", String::from_utf8_lossy(&adding_dir.stderr)),
    }
    let git_add = run_in_terminal("git add .", &current_dir);
    match git_add.status.success() {
        true => println!("Files added"),
        false => println!("Files already added - {}", String::from_utf8_lossy(&git_add.stderr)),
    }
    let git_commit = run_in_terminal(&format!("git commit -m \"{}\"", commit_message), &current_dir);
    match git_commit.status.success() {
        true => println!("Files committed"),
        false => println!("Files already committed - {}", String::from_utf8_lossy(&git_commit.stderr)),
    }
    let git_init = run_in_terminal("git init --base", &destination_dir);
    match git_init.status.success() {
        true => println!("Git initialized"),
        false => println!("Git already initialized - {}", String::from_utf8_lossy(&git_init.stderr)),
    }
    let git_push = run_in_terminal("git push local master", &current_dir);
    match git_push.status.success() {
        true => println!("Files pushed"),
        false => println!("Files already pushed - {}", String::from_utf8_lossy(&git_push.stderr)),
    }
}
