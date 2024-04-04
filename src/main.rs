use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;

//lets try using shit like bin search or bubble sorting
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
    let mut target_found = false; 
    
    if !args.recursive {
        let dl = get_dir_list(&args.dir).unwrap();
        for item in dl {
            println!("Checking {} against {}", &args.query, pathbuf_filename(&item));

        }
        //output_result(&dl);
        //println!("{}", find_query_in_vec(&args.query, &dl));
    }
    println!("Hello, world! {:?}", args.recursive);

}

fn find_query_in_vec(query: &str, target_vec: &Vec<String>) -> bool {
    // only supply vector of files, no dirs
    for item in target_vec {
        let item_file_name = {
            let path: &str = &item;
            path.split('/').last().unwrap().to_string()
        };
        println!("Checking {} against {}", &query, &item_file_name);
        if item_file_name.contains(query) {
            return true;
        }
    }
    false
}

fn get_dir_list(dir: &str) -> Result<Vec<PathBuf>, io::Error> {
    let paths = fs::read_dir(dir)?;
    let mut vec_files: Vec<String> = Vec::new();
    let mut vec_dirs: Vec<String> = Vec::new();
    
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)

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
