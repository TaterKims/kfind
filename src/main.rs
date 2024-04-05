use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Look for query in file or folder name
    #[arg(short = 'q', long)]
    query: String,

    // Where to start search, will only search given dir if supplied and -r is not true
    #[arg(short = 'd', long, default_value_t = String::from("."), required = false)]
    dir: String,

    // Recursively search
    #[arg(short, long, default_value_t = false, required = false)]
    recursive: bool,

    // Maximum depth to search subdirectories
    #[arg(long, default_value_t = 0, required = false)]
    depth: i32,

    // Minimum depth to search subdirectories
    #[arg(long, default_value_t = 0, required = false)]
    mindepth: i32,

    // Extension, multiple can be supplied. Syntax for multiple [a, b, c]
    // example: -e [exe, jpg, png]
    #[arg(short = 'e', long, default_value_t = String::from("*"), required = false)]
    extension: String,

    // Look for file or dir, if not supplied will look for both
    #[arg(short = 't' , long, default_value_t = String::from("*"), required = false)]
    file_type: String,
}

fn main() {
    let args = Args::parse();
    
    let start_point = &args.dir;
    // *.txt = .*.txt
    if args.query.contains("*") {
        if let Some(star_index) = &args.query.find("*") {
            let dot_insert = star_index - 1;
            let reg_str = [&args.query[..dot_insert], ".", &args.query[dot_insert + 1..]].join("");
            let reg = Regex::new(&reg_str).unwrap();
        }
    }
    let Ok(start_contents) = get_dir_list(&start_point) else { println!("Error getting directory contents"); return };

    for i in search_files_vector(&start_contents.1, &args.query) {
        println!("{}", i);
    }
}

fn search_dir_and_subdirs(root_directory: &str, query: &str) {
    
}
fn search_files_vector(file_list: &Vec<String>, query: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in file_list {
        match item.as_str() {
            query => result.push(item.to_string()),
            _ => (),
        }
    }
    result
}

fn get_dir_list(dir: &str) -> Result<(Vec<String>, Vec<String>), io::Error> {
    // let paths = fs::read_dir(dir)?;
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error accessing directory: {}", e);
            return Err(e);
        }
    };
    let mut vec_files: Vec<String> = Vec::new();
    let mut vec_dirs: Vec<String> = Vec::new();

    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error accessing directory entry: {}", e);
                continue;
            }
        };
        let path = match entry.path().canonicalize() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error canonicalizing directory entry path: {}", e);
                continue;
            }
        };
        if path.is_dir() {
            vec_dirs.push(path.to_str().unwrap().to_string());
        } else {
            vec_files.push(path.to_str().unwrap().to_string());
            //if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            //    vec_files.push(path.to_str().unwrap().to_string());
            //}
        }
    }
    vec_dirs.sort();
    vec_files.sort();
    Ok((vec_dirs, vec_files))
}

fn output_result(result: &Vec<String>) {
    for line in result {
        //println!("Match to {} found {}", line);
    }
}
