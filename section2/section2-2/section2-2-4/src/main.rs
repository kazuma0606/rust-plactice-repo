use garde::Validate;

#[derive(Validate)]
struct Input {
    #[garde(length(min = 1))]
    title: String,

    #[garde(range(min = 1, max = 100))]
    priority: u8,
}

fn main() {
    let input = Input {
        title: "title".to_string(),
        priority: 80,
    };

    // バリデーションエラーする例
    // let input = Input {
    //     title: "".to_string(),
    //     priority: 120,
    // };
    // エラー：入力を確認してください。 priority: greater than 100
    // title: length is lower than 1   

    match input.validate() {
        Ok(_) => println!("バリデーション成功"),
        Err(e) => println!("エラー：入力を確認してください。 {}", e),
    }
}
