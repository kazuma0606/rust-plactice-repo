//クロージャの基本とキャプチャ
fn main() {
    let x: i32 = 10;
    let add_x = |n| n + x;

    println!("x + 5 = {}", add_x(5));
    println!("x + 10 = {}", add_x(10));

    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("Incremented count: {}", increment());
    println!("Incremented count: {}", increment());
    println!("Incremented count: {}", increment());

    println!("final count: {}", count);
}
