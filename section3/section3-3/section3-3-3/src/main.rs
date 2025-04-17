use anyhow::{
    Context, Result,
};

fn run() -> Result<()> {
    let data = std::fs::read_to_string("C:/Users/yoshi/rust-mastery/section3/section3-3/section3-3-3/test.txt")
        .with_context(|| "test.txtを読み込めませんでした。")?;

    let parsed:i32 = data
    .trim()
    .trim_matches('"')
    .parse()
    .with_context(|| "test.txtの内容を整数に変換できませんでした。")?;

    println!("Parsed number: {}", parsed);
    Ok(())
}

fn main() {
    run()
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });
}
