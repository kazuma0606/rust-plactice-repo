//data types

fn main() {
    let x = 42;
    let pi = 3.14;

    //pi is a float, so we need to cast it to an integer
    let product:f64 = (x as f64) * pi;

    let name = "John Doe";

    let mut message = String::from("Hello,");
    message.push_str("rust");
    println!("x * pi{}", product);
    println!("name is {}", name);
    println!("message: {}", message);
    println!("message: {}", message[6..10].to_string());
}
