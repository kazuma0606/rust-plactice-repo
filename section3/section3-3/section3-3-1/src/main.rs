fn parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}

fn double(input: &str) -> Result<i32, std::num::ParseIntError> {
    parse_number(input).map(|num| num * 2)
}

fn main() {
    let input = "5";
    match parse_number(input) {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error parsing number: {}", e),
    }
    match double(input) {
        Ok(num) => println!("Doubled number: {}", num),
        Err(e) => println!("Error doubling number: {}", e),
    }
}
