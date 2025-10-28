use std::fs;
use std::path::Path;
use std::io::{Write, Read};
use sha1::Sha1;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: vct <command>");
        return;
    }

    match args[1].as_str() {
        "make" => {
            if args.len() < 3 {
                println!("Usage: vct make <project_name>");
                return;
            }
            make(&args[2]);
        }
        "store" => {
            if args.len() < 4 || args[2] != "-m" {
                println!("Usage: vct store -m \"message\"");
                return;
            }
            store(&args[3]);
        }
        "show" => show(),
        "goto" => {
            if args.len() < 3 {
                println!("Usage: vct goto <commit_id>");
                return;
            }
            goto(&args[2]);
        }
        _ => println!("Unknown command"),
    }
}

fn make(project_name: &str) {
    let project_dir = Path::new(project_name);

    if project_dir.exists() {
        println!("Project already exists!");
        return;
    }

    fs::create_dir(project_dir).expect("Failed to create project folder");
    fs::create_dir(project_dir.join(".vct")).expect("Failed to create .vct folder");
    fs::create_dir(project_dir.join(".vct/commits")).expect("Failed to create commits folder");

    println!("Created new project '{}' and initialized vct!", project_name);
}

fn store(message: &str) {
    let mut hasher = Sha1::new();

    // Hash all files recursively
    visit_files(Path::new("."), &mut hasher);

    // Include commit message in hash
    hasher.update(message.as_bytes());

    let commit_id = hasher.digest().to_string();
    let commit_dir = Path::new(".vct/commits").join(&commit_id);
    fs::create_dir_all(&commit_dir).expect("Failed creating commit folder");

    // Copy all files recursively
    copy_files(Path::new("."), &commit_dir);

    let mut log_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(".vct/log.txt")
        .unwrap();

    writeln!(log_file, "{}: {}", commit_id, message).unwrap();
    println!("Stored! Commit ID: {}", commit_id);
}

fn visit_files(dir: &Path, hasher: &mut Sha1) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        if path.is_file() && name != "vct.exe" && name != ".vct" {
            let mut file = fs::File::open(&path).unwrap();
            let mut content = Vec::new();
            file.read_to_end(&mut content).unwrap();
            hasher.update(&content);
            hasher.update(path.to_string_lossy().as_bytes());
        } else if path.is_dir() && name != ".vct" {
            visit_files(&path, hasher);
        }
    }
}

fn copy_files(src: &Path, dest: &Path) {
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        if name == ".vct" || name == "vct.exe" {
            continue;
        }

        if path.is_file() {
            let dest_file = dest.join(path.strip_prefix(".").unwrap());
            if let Some(parent) = dest_file.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(&path, &dest_file).unwrap();
        } else if path.is_dir() {
            let dest_dir = dest.join(path.strip_prefix(".").unwrap());
            fs::create_dir_all(&dest_dir).unwrap();
            copy_files(&path, &dest_dir);
        }
    }
}

fn show() {
    let log_path = Path::new(".vct/log.txt");

    if !log_path.exists() {
        println!("No commits found!");
        return;
    }

    let mut file = fs::File::open(log_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    if content.trim().is_empty() {
        println!("No commits yet!");
        return;
    }

    println!("Commit History:");
    println!("{}", content);
}

fn goto(commit_id: &str) {
    let commit_dir = Path::new(".vct/commits").join(commit_id);

    if !commit_dir.exists() {
        println!("Commit not found!");
        return;
    }

    // Delete all files except .vct and vct.exe recursively
    delete_files(Path::new("."));

    // Copy files from commit folder recursively
    restore_files(&commit_dir, Path::new("."));

    println!("Restored commit {}", commit_id);
}

fn delete_files(dir: &Path) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        if name == ".vct" || name == "vct.exe" {
            continue;
        }

        if path.is_file() {
            fs::remove_file(&path).unwrap();
        } else if path.is_dir() {
            delete_files(&path);
            fs::remove_dir(&path).unwrap();
        }
    }
}

fn restore_files(src: &Path, dest: &Path) {
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let relative = path.strip_prefix(src).unwrap();
        let dest_path = dest.join(relative);
        
        if path.is_file() {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(&path, &dest_path).unwrap();
        } else if path.is_dir() {
            fs::create_dir_all(&dest_path).unwrap();
            restore_files(&path, &dest_path);
        }
    }
}
