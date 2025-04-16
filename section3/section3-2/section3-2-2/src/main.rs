//イテレータとの組み合わせ

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<_> = numbers
        .iter()
        .map(|&x| x * 2)
        .collect();
    println!("Doubled numbers: {:?}", doubled);

    let even: Vec<_> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("Even numbers: {:?}", even);

    let sum: i32 = numbers
        .iter()
        .fold(0, |acc, x| acc + x);
    println!("Sum of numbers: {}", sum);

    let product: i32 = numbers
        .iter()
        .product();
    println!("Product of numbers: {}", product);
}
