use std::collections::HashMap;

use anyhow::Ok;
use axum::{
    body::{Body, Bytes},
    extract::{rejection::JsonRejection, Multipart, Path, Query, Request},
    middleware::{from_fn, map_request, Next},
    response::{self, Response, Result},
    routing::{get, post, Route},
    serve, Form, Json, Router,
};

use axum_test::{
    multipart::{MultipartForm, Part},
    TestServer,
};
use http::{method, request, HeaderMap, HeaderValue, Method, StatusCode, Uri};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello World" }));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

#[tokio::test]
async fn test_axum() {
    let app = Router::new().route("/", get(|| async { "Hello World" }));
    let server = TestServer::new(app).unwrap();
    let response = server.get("/").await;

    response.assert_status_ok();
    response.assert_text("Hello World")
}

#[tokio::test]
async fn test_method_routing() {
    async fn hello_world() -> String {
        "Hello World".to_string()
    }
    let app = Router::new()
        .route("/get", get(hello_world))
        .route("/post", post(hello_world));

    let server = TestServer::new(app).unwrap();
    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Hello World");

    let response = server.post("/post").await;
    response.assert_status_ok();
    response.assert_text("Hello World")
}

#[tokio::test]
async fn test_request() {
    async fn hello_world(request: Request) -> String {
        format!("Hello {}", request.method())
    }

    let app = Router::new()
        .route("/get", get(hello_world))
        .route("/post", post(hello_world));

    let server = TestServer::new(app).unwrap();
    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Hello GET");
}

#[tokio::test]
async fn test_extractor() {
    async fn route(uri: Uri, method: Method) -> String {
        format!("Hello {} {}", method.as_str(), uri.path())
    }

    let app = Router::new().route("/uri", get(route));

    let server = TestServer::new(app).unwrap();
    let response = server.get("/uri").await;
    response.assert_status_ok();
    response.assert_text("Hello GET /uri");
}

#[tokio::test]
async fn test_query_extractor() {
    async fn route(Query(params): Query<HashMap<String, String>>) -> String {
        format!("Hello {}", params.get("name").unwrap())
    }
    let app = Router::new().route("/query", get(route));
    let server = TestServer::new(app).unwrap();
    let response = server.get("/query").add_query_param("name", "anin").await;
    response.assert_status_ok();
    response.assert_text("Hello anin");
}

#[tokio::test]
async fn test_header() {
    async fn route(headers: HeaderMap) -> String {
        format!("Hello {}", headers["name"].to_str().unwrap())
    }

    let app = Router::new().route("/query", get(route));
    let server = TestServer::new(app).unwrap();
    let response = server.get("/query").add_query_param("name", "anin").await;
    response.assert_status_ok();
    response.assert_text("Hello anin");
}

#[tokio::test]
async fn test_path_param_extractor() {
    async fn route(Path((id, id_category)): Path<(String, String)>) -> String {
        format!("Product {}, Category {}", id, id_category)
    }

    let app = Router::new().route("/products/{id}/categories/{id_category}", get(route));
    let server = TestServer::new(app).unwrap();
    let response = server.get("/products/1/categories/2").await;
    response.assert_status_ok();
    response.assert_text("Product 1, Category 2");
}

#[tokio::test]
async fn test_string_body_extractor() {
    async fn route(body: String) -> String {
        format!("Body {}", body)
    }

    let app = Router::new().route("/post", post(route));
    let server = TestServer::new(app).unwrap();
    let response = server.post("/post").text("This is Body").await;
    response.assert_status_ok();
    response.assert_text("Body This is Body");
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[tokio::test]
async fn test_json_body_extractor() {
    async fn route(Json(request): Json<LoginRequest>) -> String {
        format!("Hello {}", request.username)
    }

    let app = Router::new().route("/post", post(route));
    let request = LoginRequest {
        username: "Aninditya".to_string(),
        password: "QS".to_string(),
    };

    let server = TestServer::new(app).unwrap();
    let response = server.post("/post").json(&request).await;
    response.assert_status_ok();
    response.assert_text("Hello Aninditya");
}

#[tokio::test]
async fn test_json_body_error() {
    async fn route(payload: Result<Json<LoginRequest>, JsonRejection>) -> String {
        match payload {
            Ok(request) => {
                format!("Hello {}", request.username)
            }
            Err(error) => {
                format!("Error: {}", error)
            }
        }
    }
    let app = Router::new().route("/post", post(route));
    let request = LoginRequest {
        username: "Aninditya".to_string(),
        password: "QS".to_string(),
    };

    let server = TestServer::new(app).unwrap();

    let response = server.post("/post").json(&request).await;
    response.assert_status_ok();
    response.assert_text("Hello Aninditya");

    let response = server.post("/post").text("tidak valid").await;
    response.assert_status_ok();
    response.assert_text("Error: Expected request with `Content-Type: application/json`");
}

#[tokio::test]
async fn test_response() {
    async fn route(request: Request) -> Response {
        Response::builder()
            .status(StatusCode::OK)
            .header("X-Owner", "aninditya")
            .body(Body::from(format!("Hello {}", request.method())))
            .unwrap()
    }
    let app = Router::new().route("/get", get(route));

    let server = TestServer::new(app).unwrap();
    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Hello GET");
    response.assert_header("X-Owner", "aninditya");
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
}

#[tokio::test]
async fn test_response_json() {
    async fn route() -> Json<LoginResponse> {
        Json(LoginResponse {
            token: "TOKEN".to_string(),
        })
    }
    let app = Router::new().route("/get", get(route));
    let server = TestServer::new(app).unwrap();
    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text_contains("TOKEN");
}

#[tokio::test]
async fn test_response_tupple() {
    async fn hello_world() -> (Response<()>, Json<LoginResponse>) {
        (
            Response::builder()
                .status(StatusCode::OK)
                .header("X-Owner", "Eko")
                .body(())
                .unwrap(),
            Json(LoginResponse {
                token: "token".to_string(),
            }),
        )
    }

    let app = Router::new().route("/get", get(hello_world));

    let server = TestServer::new(app).unwrap();
    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("{\"token\":\"token\"}");
    response.assert_header("X-Owner", "Eko");
}

#[tokio::test]
async fn test_response_tupple3() {
    async fn hello_world() -> (StatusCode, HeaderMap, Json<LoginResponse>) {
        let mut header = HeaderMap::new();
        header.insert("X-Owner", HeaderValue::from_str("Eko").unwrap());
        (
            StatusCode::OK,
            header,
            Json(LoginResponse {
                token: "token".to_string(),
            }),
        )
    }

    let app = Router::new().route("/get", get(hello_world));

    let server = TestServer::new(app).unwrap();
    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("{\"token\":\"token\"}");
    response.assert_header("X-Owner", "Eko");
}

#[tokio::test]
async fn test_form_request() {
    async fn route(Form(form): Form<LoginRequest>) -> String {
        format!("Hello {}", form.username)
    }

    let request = LoginRequest {
        username: "Aninditya".to_string(),
        password: "QS".to_string(),
    };

    let app = Router::new().route("/post", post(route));
    let server = TestServer::new(app).unwrap();
    // let response = server
    //     .post("/post")
    //     .form(&LoginRequest {
    //         username: "anin".to_string(),
    //         password: "qs".to_string(),
    //     })
    //     .await;
    let response = server.post("/post").form(&request).await;
    response.assert_status_ok();
    response.assert_text("Hello Aninditya");
}

#[tokio::test]
async fn test_multipart() {
    async fn hello_world(mut payload: Multipart) -> String {
        let mut profile: Bytes = Bytes::new();
        let mut username: String = "".to_string();

        while let Some(field) = payload.next_field().await.unwrap() {
            if field.name().unwrap_or("") == "profile" {
                profile = field.bytes().await.unwrap()
            } else if field.name().unwrap_or("") == "username" {
                username = field.text().await.unwrap()
            }
        }

        assert!(profile.len() > 0);
        format!("Hello {}", username)
    }

    let app = Router::new().route("/post", post(hello_world));

    let request = MultipartForm::new()
        .add_text("username", "Eko")
        .add_text("password", "rahasia")
        .add_part("profile", Part::bytes(Bytes::from("Contoh")));

    let server = TestServer::new(app).unwrap();
    let response = server.post("/post").multipart(request).await;
    response.assert_status_ok();
    response.assert_text("Hello Eko");
}

// Middleware - Tower
async fn log_middleware(request: Request, next: Next) -> Response {
    println!("Receive request {} {}", request.method(), request.uri());
    let response = next.run(request).await;
    println!("Send response {}", response.status());
    response
}

async fn request_id_middleware<T>(mut request: Request<T>) -> Request<T> {
    let request_id = "random_id";
    request
        .headers_mut()
        .insert("X-Request-Id", request_id.parse().unwrap());
    request
}

#[tokio::test]
async fn test_middleware() {
    async fn hello_world(method: Method, header_map: HeaderMap) -> String {
        println!("Execute handler");
        let request_id = header_map.get("X-Request-Id").unwrap().to_str().unwrap();
        format!("Hello {} {}", method, request_id)
    }

    let app = Router::new()
        .route("/get", get(hello_world))
        .layer(map_request(request_id_middleware))
        .layer(from_fn(log_middleware));

    let server = TestServer::new(app).unwrap();
    let response = server.get("/get").add_header("Cookie", "name=Eko").await;
    response.assert_status_ok();
    response.assert_text("Hello GET 12345");
}
