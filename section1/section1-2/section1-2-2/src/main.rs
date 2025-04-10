//Option type and Result type
fn find_even(numbers: Vec<i32>) -> Option<i32> {
    for n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn devide(a:i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("{:?}", find_even(vec![1, 3, 5]));
    println!("{:?}", devide(10, 0));
    println!("{:?}", devide(10, 2));
    println!("{:?}", devide(10, 2).unwrap()); //安全に値を取り出す
}
