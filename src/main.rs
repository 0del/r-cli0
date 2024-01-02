use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);
    match result {
        Ok(c) => {println!("file content: {}", c)}
        Err(e) => {panic!("err: {}", e)}
    }
}
