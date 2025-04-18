fn print_debug<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}

fn compare<T,U>(a: T, b: U) -> bool 
where 
    T: PartialEq<U>,
{
    a == b
}

fn main() {
    let a = 5;
    let b = 5;
    let c = "Hello";
    let d = "Hello";

    println!("a == b: {}", compare(a, b));
    println!("c == d: {}", compare(c, d));

    print_debug(a);
    print_debug(c);
    print_debug(vec![1, 2, 3]);
    print_debug("Hello, world!");
    print_debug((1, 2, 3));
}
