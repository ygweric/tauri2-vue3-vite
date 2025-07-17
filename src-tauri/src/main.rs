use hyper::body::HttpBody;
use once_cell::sync::Lazy;
use reqwest::Method;
use reqwest::{Client, Response};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::{error::Error, result::Result};
use warp::Filter;

// 使用 Arc 和 Mutex 包装 u16 类型来允许跨线程共享和修改端口信息
static PORT: Lazy<Arc<Mutex<u16>>> = Lazy::new(|| Arc::new(Mutex::new(0)));

#[derive(Serialize)]
struct ApiResponse {
    status: u16,
    body: serde_json::Value,
}
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // chrome_server_app_lib::run();

    // 使用 block_on 来运行异步代码
    tauri::async_runtime::block_on(async {
        // 设置端口和初始值
        let mut port = 5200;

        // 检查端口是否可用
        while !is_port_available(port).await {
            port += 1;
        }

        *PORT.lock().unwrap() = port; // 更新端口值

        println!("port: {}", port);

        // 监听 POST 请求，转发请求
        let forward_route = warp::post()
            .and(warp::path("forward"))
            .and(warp::body::json())
            .and_then(|params: ForwardParams| async move {
                let result: Result<Box<dyn warp::Reply>, warp::Rejection> =
                    match forward_request(&params).await {
                        Ok(response) => {
                            let status = response.status().as_u16();
                            let body: serde_json::Value = serde_json::from_str(
                                &response.text().await.unwrap_or_else(|_| "".to_string()),
                            )
                            .unwrap_or_default();

                            let api_response = ApiResponse { status, body };
                            Ok(Box::new(warp::reply::json(&api_response)))
                        }
                        Err(e) => {
                            println!("Failed to forward request: {}", e);
                            let error_response = ErrorResponse {
                                error: e.to_string(),
                            };

                            // 将错误响应实例序列化为 JSON 并返回
                            Ok(Box::new(warp::reply::json(&error_response)))
                        }
                    };
                result
            });

        // 创建服务并绑定到指定地址
        let server = warp::serve(forward_route);

        // 启动服务器并监听指定端口
        let server_future = server.run(([127, 0, 0, 1], port));

        // 启动后台服务器，确保 Future 被正确驱动
        tokio::spawn(async move {
            server_future.await;
        });

        // // 创建系统托盘
        // let tray_menu = SystemTrayMenu::new()
        //     .add_item(CustomMenuItem::new("quit", "Quit"));

        // let tray = SystemTray::new().with_menu(tray_menu);

        // tauri::Builder::default()
        //     .system_tray(tray)
        //     .on_window_event(|_app, _event| {})
        //     .invoke_handler(tauri::generate_handler![get_port])
        //     .build()
        //     .run()
        //     .await?;
        tauri::Builder::default()
            .on_window_event(|_app, _event| {})
            .invoke_handler(tauri::generate_handler![get_port])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");

        Ok(())
    })
}

fn string_to_method(method_str: &str) -> Method {
    match method_str.to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "PATCH" => Method::PATCH,
        "OPTIONS" => Method::OPTIONS,
        _ => Method::OPTIONS, // Return an error message for invalid method
    }
}

// 需要传递的参数
#[derive(serde::Deserialize)]
struct ForwardParams {
    target_url: String,
    target_method: String,
    target_body: String,
    target_header: Option<String>,
}

async fn forward_request(params: &ForwardParams) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    let method = string_to_method(&params.target_method);
    let mut req_builder = client
        .request(method, &params.target_url)
        .body(params.target_body.clone());

    if let Some(header) = &params.target_header {
        req_builder = req_builder.header("Custom-Header", header);
    }

    let res = req_builder.send().await?;
    println!("Forwarded request: {}", res.status());
    Ok(res)
}

async fn is_port_available(port: u16) -> bool {
    // 简单的端口检测
    let listener = std::net::TcpListener::bind(("127.0.0.1", port));
    listener.is_ok()
}

// 返回当前端口
#[tauri::command]
#[allow(dead_code)]
fn get_port() -> String {
    PORT.lock().unwrap().to_string() // 动态返回当前端口前端口（此处可以更动态化地获取当前端口）
}
