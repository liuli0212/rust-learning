//! Web服务器示例
//!
//! 演示使用Axum框架构建HTTP服务器

#[cfg(feature = "web")]
use std::net::SocketAddr;

/// 基本HTTP服务器
#[cfg(feature = "web")]
pub async fn basic_http_server() {
    println!("  === 基本HTTP服务器 ===");

    use axum::{
        routing::{get, post},
        Json, Router,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    struct CreateUser {
        name: String,
        email: String,
    }

    #[derive(Serialize)]
    struct User {
        id: u32,
        name: String,
        email: String,
    }

    // 处理GET请求
    async fn get_user() -> Json<User> {
        Json(User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        })
    }

    // 处理POST请求
    async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
        Json(User {
            id: 1,
            name: payload.name,
            email: payload.email,
        })
    }

    // 健康检查
    async fn health_check() -> &'static str {
        "OK"
    }

    // 构建路由
    let _app: Router = Router::new()
        .route("/user", get(get_user))
        .route("/users", post(create_user))
        .route("/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("  服务器启动在: http://{}", addr);

    // 启动服务器（注释掉，避免实际运行）
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

/// 路由参数
#[cfg(feature = "web")]
pub async fn route_parameters() {
    println!("  === 路由参数 ===");

    use axum::{
        extract::{Path, Query},
        routing::get,
        Router,
    };
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Pagination {
        page: Option<u32>,
        limit: Option<u32>,
    }

    // 路径参数
    async fn get_user_by_id(Path(id): Path<u32>) -> String {
        format!("获取用户 ID: {}", id)
    }

    // 查询参数
    async fn list_users(Query(params): Query<Pagination>) -> String {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(10);
        format!("用户列表 - 页码: {}, 每页: {}", page, limit)
    }

    let _app: Router = Router::new()
        .route("/users/:id", get(get_user_by_id))
        .route("/users", get(list_users));

    println!("  路由参数示例已准备就绪");
}

/// 中间件示例
#[cfg(feature = "web")]
pub async fn middleware_examples() {
    println!("  === 中间件示例 ===");

    use axum::{
        http::{Request, StatusCode},
        middleware::{self, Next},
        response::Response,
        routing::get,
        Router,
    };

    // 自定义中间件 - 记录请求
    async fn log_middleware(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
        println!("  请求: {} {}", req.method(), req.uri());
        let response = next.run(req).await;
        println!("  响应状态: {}", response.status());
        Ok(response)
    }

    // 自定义中间件 - 认证
    async fn auth_middleware(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
        // 简单的认证检查
        if let Some(auth_header) = req.headers().get("authorization") {
            if auth_header.to_str().unwrap_or("").starts_with("Bearer ") {
                return Ok(next.run(req).await);
            }
        }
        Err(StatusCode::UNAUTHORIZED)
    }

    async fn protected_route() -> &'static str {
        "受保护的资源"
    }

    let _app: Router = Router::new()
        .route("/protected", get(protected_route))
        .layer(middleware::from_fn(auth_middleware))
        .layer(middleware::from_fn(log_middleware));

    println!("  中间件示例已准备就绪");
}

/// JSON处理
#[cfg(feature = "web")]
pub async fn json_handling() {
    println!("  === JSON处理 ===");

    use axum::{
        extract::Json,
        routing::post,
        Router,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct RequestData {
        message: String,
        count: u32,
    }

    #[derive(Serialize)]
    struct ResponseData {
        received: String,
        processed: u32,
        timestamp: String,
    }

    async fn process_json(Json(payload): Json<RequestData>) -> Json<ResponseData> {
        let processed = payload.count * 2;
        Json(ResponseData {
            received: payload.message,
            processed,
            timestamp: chrono::Local::now().to_string(),
        })
    }

    let _app: Router = Router::new().route("/process", post(process_json));

    println!("  JSON处理示例已准备就绪");
}

/// 错误处理
#[cfg(feature = "web")]
pub async fn error_handling() {
    println!("  === Web错误处理 ===");

    use axum::{
        extract::Path,
        http::StatusCode,
        response::{IntoResponse, Response},
        routing::get,
        Json, Router,
    };
    use serde::Serialize;

    #[derive(Serialize)]
    struct ErrorResponse {
        error: String,
        code: u32,
    }

    // 自定义错误类型
    enum AppError {
        NotFound,
        InternalServerError,
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            let (status, error_message) = match self {
                AppError::NotFound => (StatusCode::NOT_FOUND, "资源未找到".to_string()),
                AppError::InternalServerError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "服务器内部错误".to_string(),
                ),
            };

            let body = Json(ErrorResponse {
                error: error_message,
                code: status.as_u16() as u32,
            });

            (status, body).into_response()
        }
    }

    async fn get_user(Path(id): Path<u32>) -> Result<Json<String>, AppError> {
        if id == 0 {
            Err(AppError::NotFound)
        } else {
            Ok(Json(format!("用户 ID: {}", id)))
        }
    }

    let _app: Router = Router::new().route("/user/:id", get(get_user));

    println!("  错误处理示例已准备就绪");
}

/// 静态文件服务
#[cfg(feature = "web")]
pub async fn static_files() {
    println!("  === 静态文件服务 ===");

    use axum::{
        routing::get,
        Router,
    };

    // 简单的文件内容
    async fn get_html() -> &'static str {
        r#"
        <!DOCTYPE html>
        <html>
        <head><title>Rust Web Server</title></head>
        <body>
            <h1>Hello from Rust!</h1>
            <p>This is a simple HTML response.</p>
        </body>
        </html>
        "#
    }

    let _app: Router = Router::new().route("/", get(get_html));

    println!("  静态文件服务示例已准备就绪");
}

/// 状态共享
#[cfg(feature = "web")]
pub async fn shared_state() {
    println!("  === 状态共享 ===");

    use axum::{
        extract::State,
        routing::get,
        Router,
    };
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[derive(Clone)]
    struct AppState {
        counter: Arc<RwLock<u32>>,
    }

    async fn get_counter(State(state): State<Arc<AppState>>) -> String {
        let count = *state.counter.read().await;
        format!("当前计数: {}", count)
    }

    async fn increment_counter(State(state): State<Arc<AppState>>) -> String {
        let mut count = state.counter.write().await;
        *count += 1;
        format!("计数已增加到: {}", *count)
    }

    let shared_state = Arc::new(AppState {
        counter: Arc::new(RwLock::new(0)),
    });

    let _app: Router = Router::new()
        .route("/counter", get(get_counter))
        .route("/increment", get(increment_counter))
        .with_state(shared_state);

    println!("  状态共享示例已准备就绪");
}

/// 运行所有Web服务器示例
#[cfg(feature = "web")]
pub async fn run_examples() {
    basic_http_server().await;
    route_parameters().await;
    middleware_examples().await;
    json_handling().await;
    error_handling().await;
    static_files().await;
    shared_state().await;
}
