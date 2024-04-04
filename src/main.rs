use clap::Parser;
use std::fs;


//lets try using shit like bin search or bubble sorting
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

//TODO: 
// add wildcards like * so I can search for *ss and find ./piss and ./ass
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
        let dl = get_dir_list(&args.dir);
        for d in dl {
            println!("{}", &d);
            //if find_query_in_vec(&args.query, &d) {
            //    println!("Found query {}: {}", &args.query, &d);
            //    //output_result(&vec![d]);
            //    //output_result(&vec![d]);
            //}
        }
        //output_result(&dl);
        //println!("{}", find_query_in_vec(&args.query, &dl));
    }
    println!("Hello, world! {:?}", args.recursive);

}

fn find_query_in_vec(query: &str, target: &str) -> bool {
    let target_file_name = {
        let path: &str = &target;
        path.split('/').last().unwrap().to_string()
    };
    target_file_name.contains(query)
}

fn get_dir_list(dir: &str) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    let mut vec: Vec<String> = Vec::new();
    for path in paths {
        vec.push(path.unwrap().path().display().to_string());
    }
    vec.sort();
    // without sort
    // .\.git
    // .\.gitignore
    // .\Cargo.lock
    // .\Cargo.toml
    // .\src
    // .\target
    vec

}


fn output_result(result: &Vec<String>) {
    for line in result {
        //println!("Match to {} found {}", line);
    }
}
