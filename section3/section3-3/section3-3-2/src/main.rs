use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("入力を確認してください。文字列は変換できません。: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("入力が負の値です。")]
    NegativeValue,
}

fn safe_parse(input: &str) -> Result<i32, AppError> {
    let n: i32 = input.parse()?;
    match n {
        n if n < 0 => Err(AppError::NegativeValue),
        _ => Ok(n),
    }
}

fn main() {
    let input = "5";
    match safe_parse(input) {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error parsing number: {}", e),
    }

    let input = "-5";
    match safe_parse(input) {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error parsing number: {}", e),
    }

    let input = "abc";
    match safe_parse(input) {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error parsing number: {}", e),
    }
}
