
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

fn find<P: AsRef<Path>>(root: P, regex: &Regex) -> Result<Vec<String>, Box<dyn Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, &mut matches)?;
    Ok(matches)
}

fn walk_tree(path: &Path, regex: &Regex, matches: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches)?;
            } else {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name) = file_name.to_str() {
                        if regex.is_match(file_name) {
                            matches.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <input> <pattern>", args[0]);
        process::exit(1);
    }

    let pattern = &args[2];

    let regex = match Regex::new(pattern) {
        Ok(regex) => regex,
        Err(err) => {
            println!("Invalid regex: {}", err);
            process::exit(1);
        }
    };

    match find(&args[1], &regex) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("No matches found");
            } else {
                println!("Matches:");
                for match_ in matches {
                    println!("{}", match_);
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
    
}
