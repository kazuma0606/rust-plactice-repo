#![allow(unused)]
//HKT emulation

trait Higher {
    type Applied<T>;
}

struct OptionH;
impl Higher for OptionH {
    type Applied<T> = Option<T>;
}

struct VecH;
impl Higher for VecH {
    type Applied<T> = Vec<T>;
}

fn transfrom<H, T, U, F>(conteiner: H::Applied<T>, f: F) -> H::Applied<U>
where
    H: Higher,
    F: Fn(T) -> U,
{
    todo!()
}

fn transform_option<T, U, F>(conteiner: Option<T>, f: F) -> Option<U>
where
    F: Fn(T) -> U,
{
    conteiner.map(f)
}

fn main() {
    let result = transform_option(Some(5), |x| x.to_string());
    println!("{:?}", result); // Some("5")
    let result = transform_option(None, |x: String| x.to_string());
    println!("{:?}", result); // None
    let result = transform_option(Some(5), |x| x + 1);
    println!("{:?}", result); // Some(6)
}
