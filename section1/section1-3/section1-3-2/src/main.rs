use clap::Parser;
use colored::*;

#[derive(Parser)]
struct  Args {
    #[arg(short, long)]
    name: String, //パラメータとして受け取る
}

fn main() {
    let args = Args::parse();
    println!("Hi, {}!", format!("{}", args.name).blue().bold());
}