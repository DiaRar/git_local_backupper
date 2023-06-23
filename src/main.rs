use std::env::{current_dir, args};
use std::path::Path;
use std::process::{Command, Output};
use std::io;

fn get_current_dir() -> String {
    let current_dir = current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();
    current_dir.to_string()
}
fn run_in_terminal(command: &str, cwd: &str) -> io::Result<Output> {
    Command::new("cmd")
        .args(&["/C", command])
        .current_dir(cwd)
        .output()

}
fn main() {
    let current_dir = get_current_dir();
    let mut destination_dir = current_dir.clone();
    destination_dir.replace_range(0..1, "D");
    match Path::new(&destination_dir).exists() {
        true => println!("Destination directory exists"),
        false => {
            run_in_terminal(&format!("mkdir {}", &destination_dir), &current_dir).expect("Failed to create directory");
        }
    }
    let commit_message = args().nth(1).unwrap_or("Empty".to_string());
    let adding_dir = run_in_terminal(&format!("git remote add local {}", &destination_dir), &current_dir);
    match adding_dir {
        Ok(out) => println!("Remote added  - {:#?}", out),
        Err(_) => println!("Remote already exists"),
    }
    run_in_terminal("git add .", &current_dir).expect("Failed to add files");
    run_in_terminal(&format!("git commit -m \"{}\"", commit_message), &current_dir).expect("Failed to commit");
    run_in_terminal("git init --base", &destination_dir).expect("Failed to push");
    run_in_terminal("git push local master", &current_dir).expect("Failed to push");
}
