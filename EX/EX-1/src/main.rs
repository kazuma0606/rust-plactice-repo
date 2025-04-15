fn main() {
    println!("Hello, world!");
    // println!("2 + 3 = {}", add(2, 3));
}

//①: testでしか使わない関数は#[allow(dead_code)]をつけておく
//②: もしくはpub fnにしておく
//③:もしくは普通にmainで使う
#[allow(dead_code)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic]
    fn test_add_fail() {
        assert_eq!(add(2, 3), 4); // 実際は 5 になるはず
    }
}

