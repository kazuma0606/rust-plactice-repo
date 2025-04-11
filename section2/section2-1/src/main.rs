use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
    extract::Path,
    Json,
    routing::post,
};

use serde::{Serialize, Deserialize};

use utoipa::{OpenApi, ToSchema};

#[derive(Deserialize, ToSchema)]  // ToSchema を追加
struct GreetRequest {
    name: String,
}

#[derive(Serialize, ToSchema)]    // ToSchema を追加
struct GreetResponse {
    message: String,
}

/// JSON形式の挨拶エンドポイント
#[utoipa::path(
    post,
    path = "/greet_json",
    request_body = GreetRequest,
    responses(
        (status = 200, description = "挨拶が正常に生成されました", body = GreetResponse)
    )
)]
async fn greet_json(Json(payload): Json<GreetRequest>) -> Json<GreetResponse> {
    let message = format!("Hello, {}!", payload.name);
    Json(GreetResponse { message })
}

/// シンプル挨拶エンドポイント
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "シンプルな挨拶メッセージ", body = String)
    )
)]
async fn hello() -> &'static str {
    "Hello, World!"
}

/// パスパラメータを使用した挨拶エンドポイント
#[utoipa::path(
    get,
    path = "/greet/{name}",
    responses(
        (status = 200, description = "指定された名前への挨拶", body = String)
    ),
    params(
        ("name" = String, Path, description = "挨拶する相手の名前")
    )
)]
async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

/// APIドキュメンテーションを生成する構造体
#[derive(OpenApi)]
#[openapi(
    paths(
        hello,
        greet,
        greet_json
    ),
    components(
        schemas(GreetRequest, GreetResponse)
    ),
    tags(
        (name = "greeting", description = "挨拶関連API")
    ),
    info(
        title = "挨拶アプリケーションAPI",
        version = "1.0.0",
        description = "シンプルな挨拶を提供するRESTful API",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
struct ApiDoc;

use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello))
        .route("/greet/{name}", get(greet))
        .route("/greet_json", post(greet_json))
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Listening on {}", addr);
    println!("Swagger UI available at: http://{}/swagger-ui/", addr);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}