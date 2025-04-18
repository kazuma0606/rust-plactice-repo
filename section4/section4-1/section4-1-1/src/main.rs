fn echo<T>(value: T) -> T {
    value
}

struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self {
        Pair { a, b }
    }

    fn get_a(&self) -> &T {
        &self.a
    }

    fn get_b(&self) -> &T {
        &self.b
    }
}

fn main() {
    let x = echo(42);
    let y = echo("Hello, world!");
    let z = echo(vec![1, 2, 3]);

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {:?}", z);

    let pair = Pair::new(1, 2);
    println!("Pair a: {}", pair.get_a());
    println!("Pair b: {}", pair.get_b());
    let pair_str = Pair::new("Hello", "World");
    println!("Pair a: {}", pair_str.get_a());
    println!("Pair b: {}", pair_str.get_b());
    let pair_vec = Pair::new(vec![1, 2, 3], vec![4, 5, 6]);
    println!("Pair a: {:?}", pair_vec.get_a());
    println!("Pair b: {:?}", pair_vec.get_b());
}
