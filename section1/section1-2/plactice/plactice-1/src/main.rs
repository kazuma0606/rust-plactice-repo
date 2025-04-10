use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    pattern: String,
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.file).expect("ファイルを開けませんでした");
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            let highlight = line.replace(
                &args.pattern,
                &args.pattern.red().bold().to_string(),
            );

            println!("{}: {}", i + 1, highlight);
        }
    }
}

// PS C:\Users\yoshi\rust-mastery\section1\section1-2\plactice> cargo run -p plactice-1 -- --pattern Rust --file plactice-1/src/test.txt
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
//      Running `C:\Users\yoshi\rust-mastery\target\debug\plactice-1.exe --pattern Rust --file plactice-1/src/test.txt`
// 1: Rust is fun!
// 2: Learning Rust with clap and serde.