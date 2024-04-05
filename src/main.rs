use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;

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
    
    if args.mindepth & args.depth == 0 {
        let current_dir = &args.dir;
        let current_dir_list = get_dir_list(current_dir);
        let results = search_current_dir(&args.query, &current_dir_list.1);
        for line in results {
            println!("Found match {}", &line);
        }
    }
}

fn search_current_dir(query: &str, dir_list: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in dir_list {
        if item.contains(query) {
            result.push(item.to_string());
        }
    }
    result
}

fn get_dir_list(dir: &str) -> (Vec<String>, Vec<String>) {
    //let paths = fs::read_dir(dir)?;
    let entries = fs::read_dir(&dir).unwrap();
    let mut vec_files: Vec<String> = Vec::new();
    let mut vec_dirs: Vec<String> = Vec::new();

    for path in entries {
        let path = &path.unwrap().path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                println!("{}", &dir_name);
                vec_dirs.push(dir_name.to_string());
            }
        }  else {
            vec_files.push(path.to_str().unwrap().to_string());
        }
    }
    vec_dirs.sort();
    vec_files.sort();
    (vec_dirs, vec_files)

}

fn output_result(result: &Vec<String>) {
    for line in result {
        //println!("Match to {} found {}", line);
    }
}
