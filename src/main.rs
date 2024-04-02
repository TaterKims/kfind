use clap::Parser;

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
    #[arg(short, long, default_value_t = true, required = false)]
    recursive: bool,

    // Extension, multiple can be supplied. Syntax for multiple [a, b, c]
    // example: -e [exe, jpg, png]
    #[arg(short, long, required = false)]
    extension: Option<String>,

    // Look for file or dir, if not supplied will look for both
    #[arg(short, long)]
    file_type: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!("Hello, world! {:?}", args.query);

}
