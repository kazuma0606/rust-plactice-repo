use EX_2::{fetch_data, ApiResponse};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async  fn test_fetch_data_returns_response() {
    let mock_server = MockServer::start().await;

    let response = ResponseTemplate::new(200)
        .set_body_json(ApiResponse {
            message: "Hello from mock!".to_string(),
        });

    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    let result = fetch_data(&mock_server.uri()).await.unwrap();
    assert_eq!{result.message, "Hello from mock!".to_string()};
}