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

    // Where to search, will only search given dir if supplied and -r is not true
    #[arg(short = 'd', long, default_value_t = String::from("."), required = false)]
    dir: String,

    // Recursively search
    #[arg(short, long, default_value_t = false, required = false)]
    recursive: bool,

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
    
    if !args.recursive {
        let dl = get_dir_list(&args.dir).unwrap();
        for item in dl {
            println!("{}", &item);
            //println!("Checking {} against {}", &args.query, pathbuf_filename(&item));
        }
    }
}

fn find_query_in_vec(query: &str, target_vec: &Vec<PathBuf>) -> bool {
    // only supply vector of files, no dirs
    for item in target_vec {
        let item_file_name = pathbuf_filename(&item);
        println!("Checking {} against {}", &query, &item_file_name);
        if item_file_name.contains(query) {
            return true;
        }
    }
    false
}

//fn sort_dir_entires(dir_list: &Vec<PathBuf>) -> (Vec<PathBuf>, Vec<PathBuf>) {
//    let mut vec_files: Vec<String> = Vec::new();
//    let mut vec_dirs: Vec<String> = Vec::new();
//
//    for item in dir_list {
//        if item.is_dir() {
//            vec_dirs.push(&item.unwrap().to_str());
//        }
//    }
//    (vec_dirs, vec_files)
//}

fn get_dir_list(dir: &str) -> (Vec<String>, Vec<String>) {
    //let paths = fs::read_dir(dir)?;
    let entries = fs::read_dir(&dir).unwrap();
    let mut vec_files: Vec<String> = Vec::new();
    let mut vec_dirs: Vec<String> = Vec::new();

    for path in entries {
        let path = &path.unwrap().path();
        if path.is_dir() {
            vec_dirs.push(path.to_str().unwrap().to_string());   
        }  else {
            vec_files.push(path.to_str().unwrap().to_string());
        }
    }
    vec_dirs.sort();
    vec_files.sort();
    (vec_dirs, vec_files)

}

fn pathbuf_filename(pathbuf: &PathBuf) -> String {
    pathbuf
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}


fn output_result(result: &Vec<String>) {
    for line in result {
        //println!("Match to {} found {}", line);
    }
}
