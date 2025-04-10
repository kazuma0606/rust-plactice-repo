use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
struct  Args {
    #[arg(short, long)]
    name: String, //パラメータとして受け取る
}

// `name.txt`に名前を書き込む関数
// 中身がなければ新規作成、あれば上書き
fn write_name_to_file(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("section1/section1-3/section1-3-3/src/name.txt")?;
    writeln!(file, "{}", args.name)?;

    println!("{}", format!("{}さんの名前を保存しました！", args.name).blue());
    Ok(())
}

fn main() {
    let args = Args::parse();
     write_name_to_file(&args).unwrap_or_else(|err| {
        eprintln!("Error writing to file: {}", err);
        std::process::exit(1);
    });
}