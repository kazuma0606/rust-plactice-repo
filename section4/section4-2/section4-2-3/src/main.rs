#![allow(unused)]

struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}
impl<T, const N: usize> Matrix<T, N, N> {
    fn is_square(&self) -> Result<bool, String> {
        // true
        match self {
            Matrix { data } if data.len() == N && data[0].len() == N => Ok(true),
            _ => Err("Matrix is not square".to_string()),
        }
    }
}
fn main() {
    let mat = Matrix {
        data: [[1, 2], [3, 4]],
    };
    println!("Is square: {:?}", mat.is_square());
    let mat2 = Matrix {
        data: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
    };
    println!("Is square: {:?}", mat2.is_square());

    // 2x3の行列は正方行列ではない
    let mat3 = Matrix {
        data: [[1, 2, 3], [4, 5, 6]],
    };
    // 正方行列でない場合はエラーを返す
    // println!("Is square: {:?}", mat3.is_square());
}
