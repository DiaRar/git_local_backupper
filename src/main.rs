use std::env::{current_dir, args};
use std::path::Path;
use std::process::{Command, Output};
struct Dir {
    dir: String,
    dir_name: String,
    parent: String,
}
fn get_current_dir() -> Dir {
    let current_dir = current_dir().unwrap();
    let parent = current_dir.parent().unwrap();
    let current_dir_name = current_dir.file_name().unwrap();
    let current_dir = current_dir.to_str().unwrap();
    
    Dir {
        dir: current_dir.to_string(),
        dir_name: current_dir_name.to_str().unwrap().to_string(),
        parent: parent.to_str().unwrap().to_string(),
    }
}
fn run_in_terminal(command: &str, cwd: &str) -> Output {
    Command::new("cmd")
        .args(&["/C", command])
        .current_dir(cwd)
        .output()
        .expect("IO Error")

}

fn main() {
    let Dir{dir: current_dir, dir_name, parent} = get_current_dir();
    let mut destination_dir = parent.clone();
    destination_dir.replace_range(0..1, "D");
    destination_dir.push_str("\\git_backups\\");
    destination_dir.push_str(&dir_name);
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
        false => println!("Remote already exists"),
    }
    let _git_add = run_in_terminal("git add .", &current_dir);
    let _git_commit = run_in_terminal(&format!("git commit -m \"{}\"", commit_message), &current_dir);
    let _git_init = run_in_terminal("git init --bare", &destination_dir);
    let git_push = run_in_terminal("git push local master", &current_dir);
    match git_push.status.success() {
        true => println!("Files pushed"),
        false => println!("Push Error \n {}", String::from_utf8_lossy(&git_push.stderr)),
    }
}
