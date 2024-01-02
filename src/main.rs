use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("can not read the file");
    for line in content.lines() {
        println!("{}", line)
    }
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
