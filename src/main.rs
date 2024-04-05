use clap::Parser;
use std::fs;
use std::io;
use std::sync::mpsc;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
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
    
    let start_point = args.dir;
    // *.txt = .*.txt
    if args.query.contains("*") {
        let reg = make_regex(&args.query);
        let (tx, rx) = mpsc::channel();
        let working = vec!["\\", "|", "/", "-"];
        let mut running = true;
        
        let handle = std::thread::spawn(move || {
            println!("Searching {} with regex {}", start_point, reg.as_str());
            tx.send(search_dir_and_subdirs_regex(&start_point, &reg)).unwrap();
            thread::sleep(Duration::from_millis(1));
        });

        let mut counter: usize = 0;
        while running {
            if handle.is_finished() {
                running = false;
            }
            println!("Searching, please wait... {}", counter);
            counter += 1;
            thread::sleep(Duration::from_secs(1));
        }

        let output = rx.recv().unwrap();
        handle.join().unwrap();
        output_result(&output, &args.query);
    
    } else {
        let output = search_dir_and_subdirs(&start_point, &args.query);
        output_result(&output, &args.query);
    }
}

fn display_working() {

}
fn get_file_name(path: &str) -> String {
    PathBuf::from(path).file_name().unwrap().to_str().unwrap().to_string()
}

fn make_regex(query_string: &str) -> Regex {
    let star_index = query_string.find("*").unwrap(); // will panic if * is query_string[0] such as *.txt
    if star_index == 0 {
        let mut new_string = String::new(); // ""
        new_string.push('.');                   // "."
        new_string.push_str(&query_string);        // ".*.txt"
        return Regex::new(&new_string).unwrap();
    }
    let dot_insert = star_index - 1;
    let reg_str = [&query_string[..dot_insert], ".", &query_string[dot_insert + 1..]].join("");
    Regex::new(&reg_str).unwrap()
}

fn search_dir_and_subdirs_regex(search_dir: &str, reg: &regex::Regex) -> Vec<String> {
    //println!("Searching {} with regex {}", search_dir, reg.as_str());
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

fn search_dir_and_subdirs(search_dir: &str, query: &str) -> Vec<String> {
    //println!("Searching {} with query {}", search_dir, query);
    let mut search_results = Vec::new();
    let search_contents = get_dir_list(&search_dir).unwrap();
    
    for item in search_contents.1 {
        let item_file_name = get_file_name(&item);
        if item_file_name.contains(&query) {
            search_results.push(item);
        }
    }
    for subdirectory in search_contents.0 {
        search_results.extend(search_dir_and_subdirs(&subdirectory, &query));
    }
    search_results
    
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

fn output_result(result: &Vec<String>, query: &str) {
    println!("Found {} results for search for '{}'", result.len(), &query);
    for line in result {
        println!("> {}", line);
    }
}
