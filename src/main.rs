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
        let reg = make_regex(&args.query);
    }
}

fn make_regex(query_string: &str) -> Regex {
    let star_index = &query_string.find("*").unwrap(); 
    let dot_insert = star_index - 1;
    let reg_str = [&query_string[..dot_insert], ".", &query_string[dot_insert + 1..]].join("");
    Regex::new(&reg_str).unwrap()
}

fn search_dir_and_subdirs_regex(search_dir: &str, reg: regex::Regex) -> Vec<String> {
    
    let mut search_results = Vec::new();
    let search_contents = get_dir_list(&search_dir).unwrap();
    
    for item in search_contents.1 {
        let item_file_name = get_file_name(&item);
        if reg.is_match(&item_file_name) { search_results.push(item);}
    }
    for subdirectory in search_contents.0 {
        search_results.extend(search_dir_and_subdirs_regex(&subdirectory, reg));
    }
    search_results
}

fn get_file_name(path: &str) -> String {
    PathBuf::from(path).file_name().unwrap().to_str().unwrap().to_string()
}

fn search_dir_and_subdirs(search_dir: &str, query: &str) -> Vec<String> {
    let mut search_results = Vec::new();
    let search_contents = get_dir_list(&search_dir).unwrap();
    
    for item in search_contents.1 {
        let item_file_name = get_file_name(&item);
        if item_file_name.contains(&query) {
            search_results.push(item);
        }
    }
    for subdirectory in search_contents.0 {
        search_results.extend(search_dir_and_subdirs_regex(&subdirectory, reg));
    }
    search_results
    
}

fn search_single_directory(directory: &str, query: &str) {
    let contents = get_dir_list(&directory).unwrap();
    let files = contents.1;
    let mut result: Vec<String> = Vec::new();
    for item in files {
        match item.as_str() {
            query => result.push(item.to_string()),
            _ => (),
        }
    }
    output_result(&result);
}

fn search_files_vector(file_list: &Vec<String>, query: &str) -> Option<Vec<String>> {
    let mut output = Vec::new();
    for item in file_list {
        let item_file_name = PathBuf::from(item).file_name().unwrap().to_str().unwrap();
        if item_file_name.contains(query) {
            output.push(item.to_string());
        }
    }
    if output.is_empty() {
        return None;
    }
    Some(output)
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
