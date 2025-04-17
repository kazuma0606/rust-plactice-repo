use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("入力を確認してください。文字列は変換できません。: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("入力が負の値です。")]
    NegativeValue,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::ParseError(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            AppError::NegativeValue => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()).into_response(),
        }
    }
}

// ハンドラー関数を修正
async fn parse_handler(Path(input): Path<String>) -> Result<String, AppError> {
    let parsed = parse_number(&input).await?;
    Ok(format!("パース結果: {}", parsed))
}

// 実際のパース処理を行う関数
async fn parse_number(input: &str) -> Result<i32, AppError> {
    let parsed = input.parse::<i32>()?;
    if parsed < 0 {
        return Err(AppError::NegativeValue);
    }
    Ok(parsed)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/parse/{input}", get(parse_handler));

    // サーバーを起動
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("サーバーを起動します: {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}