fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

fn main() {
    let double = |x| x * 2;
    let result = apply_twice(double, 5);
    println!("Result: {}", result); // 20
}
