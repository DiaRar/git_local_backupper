use std::env::{current_dir, args};
use std::path::Path;
use std::process::{Command, Output};
use argmap;
struct Dir {
    current_dir: String,
    dir_name: String,
    parent: String,
}
fn get_current_dir() -> Dir {
    let current_dir = current_dir().unwrap();
    let parent = current_dir.parent().unwrap();
    let current_dir_name = current_dir.file_name().unwrap();
    let current_dir = current_dir.to_str().unwrap();
    
    Dir {
        current_dir: current_dir.to_string(),
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
fn push(remote : &str, branch : &str, current_dir: &str) -> Output {
     run_in_terminal(&format!("git push {} {}", remote, branch), &current_dir)
}
fn add_and_commit(current_dir: &str, commit_message: &Vec<String>) {
    let _git_add = run_in_terminal("git add .", &current_dir);
    let mut commit_command = "git commit".to_string();
    for message in commit_message {
        commit_command.push_str(&format!(" -m \"{}\"", message));
    }
    let _git_commit = run_in_terminal(&commit_command, &current_dir);
}
fn get_destination_dir(parent: &str, dir_name: &str) -> String {
    let mut destination_dir = parent.to_string();
    destination_dir.replace_range(0..1, "D");
    destination_dir.push_str("\\git_backups\\");
    destination_dir.push_str(&dir_name);
    destination_dir
}
fn main() {
    let Dir{current_dir, dir_name, parent} = get_current_dir();
    let destination_dir = get_destination_dir(&parent, &dir_name);
    println!("Destination: {}", &destination_dir);
    match Path::new(&destination_dir).exists() {
        true => println!("Destination directory exists"),
        false => {
            run_in_terminal(&format!("mkdir {}", &destination_dir), &current_dir);
        }
    }
    let (_, args) = argmap::parse(args());
    let default = vec!["Empty".to_string()];
    let commit_message = args.get("commit").unwrap_or(&default);
    add_and_commit(&current_dir, commit_message);
    let adding_dir = run_in_terminal(&format!("git remote add local {}", &destination_dir), &current_dir);
    match adding_dir.status.success() {
        true => println!("Remote added"),
        false => println!("Remote already exists"),
    }
    let _git_init = run_in_terminal("git init --bare", &destination_dir);
    let local_push = push("local", "master", &current_dir);
    match local_push.status.success() {
        true => println!("Files pushed"),
        false => {
            println!("Usually it's because there is another local lmao");
            run_in_terminal("git remote rm local", &current_dir);
            add_and_commit(&current_dir, &commit_message);
            run_in_terminal(&format!("git remote add local {}", &destination_dir), &current_dir);
            let push_retry = run_in_terminal("git push local master", &current_dir);
            match push_retry.status.success() {
                true => println!("Files pushed"),
                false => panic!("Push Error \n {}", String::from_utf8_lossy(&push_retry.stderr)),
            }
        },
    }

    if let Some(remote) = args.get("remote") {
        let remote_push = push(&remote.join(""), "master", &current_dir);
        match remote_push.status.success() {
            true => println!("Files pushed to {}", &remote.join("")),
            false => println!("Origin remote not set!\n{}", String::from_utf8_lossy(&remote_push.stderr)),
        }
    }
}
