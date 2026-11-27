
use std::env;
use std::fs;
use std::process::{Command, ExitCode};

fn install_packages(packages: &[String]) -> ExitCode {
    if packages.is_empty() {
        eprintln!("Error: No packages specified.");
        return ExitCode::FAILURE;
    }

    println!("Installing packages: {}", packages.join(", "));

    let status = Command::new("sudo")
        .arg("pacman")
        .arg("-S")
        .arg("--noconfirm")
        .arg("--")
        .args(packages)
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("Packages installed successfully!");
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

        let timestamp = match line.find(']') {
            Some(pos) => line[1..pos].to_string(),
            None => continue,
        };

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

    packages.reverse();

    println!("Recently installed packages:\n");

    for (timestamp, package) in packages.iter().take(count) {
        println!("{}  {}", timestamp, package);
    }

    ExitCode::SUCCESS
}

fn print_usage(program: &str) {
    println!("Usage:");
    println!("  {} <package> [package...]", program);
    println!("  {} install <package> [package...]", program);
    println!("  {} recent [count]", program);
    println!("  {} help", program);

    println!();
    println!("Commands:");
    println!("  install   Install one or more packages");
    println!("  recent    List recently installed packages");
    println!("  help      Display this help message");

    println!();
    println!("Examples:");
    println!("  {} firefox", program);
    println!("  {} firefox git vim", program);
    println!("  {} install firefox git vim", program);
    println!("  {} recent 20", program);
}

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        print_usage("wrapper");
        return ExitCode::SUCCESS;
    }

    match args[0].as_str() {
        "install" => {
            args.remove(0);

            if args.is_empty() {
                eprintln!("Error: Missing package names.");
                return ExitCode::FAILURE;
            }

            if args.iter().any(|package| package.starts_with('-')) {
                eprintln!("Error: Invalid package name.");
                return ExitCode::FAILURE;
            }

            install_packages(&args)
        }

        "recent" => {
            if args.len() > 2 {
                eprintln!("Error: Too many arguments for recent.");
                return ExitCode::FAILURE;
            }

            let count = if args.len() == 2 {
                match args[1].parse::<usize>() {
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
            print_usage("wrapper");
            ExitCode::SUCCESS
        }

        _ => {
            if args.iter().any(|package| package.starts_with('-')) {
                eprintln!("Error: Invalid package name.");
                return ExitCode::FAILURE;
            }

            install_packages(&args)
        }
    }
}

