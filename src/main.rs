use std::env;
use std::fs;
use std::process::{Command, ExitCode};

fn install_package(package: &str) -> ExitCode {
    println!("Installing package: {}", package);

    let status = Command::new("sudo")
        .arg("pacman")
        .arg("-S")
        .arg("--noconfirm")
        .arg("--")
        .arg(package)
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("Package installed successfully!");
            ExitCode::SUCCESS
        }
        Ok(status) => {
            eprintln!("Pacman exited with status: {}", status);
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("Failed to execute pacman: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn list_recent_packages(count: usize) -> ExitCode {
    let log_path = "/var/log/pacman.log";

    let contents = match fs::read_to_string(log_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Failed to read pacman log: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let mut packages: Vec<(String, String)> = Vec::new();

    for line in contents.lines() {
        if !line.contains("[ALPM] installed ") {
            continue;
        }

        // Extract timestamp
        let timestamp = match line.find(']') {
            Some(pos) => line[1..pos].to_string(),
            None => continue,
        };

        // Extract package name and version
        if let Some(pos) = line.find("[ALPM] installed ") {
            let info = &line[pos + 17..];

            if let Some((name, _version)) = info.split_once(" (") {
                packages.push((timestamp, name.to_string()));
            }
        }
    }

    if packages.is_empty() {
        println!("No installed packages found.");
        return ExitCode::SUCCESS;
    }

    // Show newest installations first
    packages.reverse();

    println!("Recently installed packages:\n");

    for (timestamp, package) in packages.iter().take(count) {
        println!("{}  {}", timestamp, package);
    }

    ExitCode::SUCCESS
}

fn print_usage(program: &str) {
    println!("Usage:");
    println!("  {} install <package>", program);
    println!("  {} recent [count]", program);
    println!();
    println!("Commands:");
    println!("  install   Install a package using pacman");
    println!("  recent    List recently installed packages");
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // No arguments: use default install behaviour
    if args.len() < 2 {
        return install_package("default-package");
    }

    match args[1].as_str() {
        "install" => {
            if args.len() < 3 {
                eprintln!("Error: Missing package name.");
                return ExitCode::FAILURE;
            }

            if args[2].starts_with('-') {
                eprintln!("Invalid package name.");
                return ExitCode::FAILURE;
            }

            install_package(&args[2])
        }

        "recent" => {
            let count = if args.len() >= 3 {
                match args[2].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid package count.");
                        return ExitCode::FAILURE;
                    }
                }
            } else {
                10
            };

            list_recent_packages(count)
        }

        "help" | "--help" | "-h" => {
            print_usage(&args[0]);
            ExitCode::SUCCESS
        }

        _ => {
            // Treat an unrecognised first argument as a package name
            install_package(&args[1])
        }
    }
}
