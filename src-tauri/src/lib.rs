use include_dir::{include_dir, Dir};

use tauri::http::StatusCode;

// Option 1: Serve files embedded at compile time (recommended for static assets)
// Put your files in `assets/` folder in the project root
static EMBEDDED_ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../src/assets");

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn test_local_assets() -> Result<serde_json::Value, String> {
    use std::collections::HashMap;

    let mut results = HashMap::new();

    // Test qwerty.localhost
    let localhost_tests = vec![
        ("index", "http://qwerty.localhost/"),
        ("config", "http://qwerty.localhost/config.json"),
        ("favicon", "http://qwerty.localhost/favicon.ico"),
        ("logo", "http://qwerty.localhost/logo.png"),
    ];

    let mut localhost_results = HashMap::new();
    for (name, url) in localhost_tests {
        localhost_results.insert(name.to_string(), format!("URL: {}", url));
    }
    results.insert("qwerty.localhost".to_string(), localhost_results);

    // Test 127.0.0.1:3000
    let ip_tests = vec![
        ("index", "qwerty://127.0.0.1:3000/"),
        ("config", "qwerty://127.0.0.1:3000/config.json"),
        ("favicon", "qwerty://127.0.0.1:3000/favicon.ico"),
        ("logo", "qwerty://127.0.0.1:3000/logo.png"),
    ];

    let mut ip_results = HashMap::new();
    for (name, url) in ip_tests {
        ip_results.insert(name.to_string(), format!("URL: {}", url));
    }
    results.insert("127.0.0.1:3000".to_string(), ip_results);

    Ok(serde_json::to_value(results).unwrap())
}

fn handle_local_assets(
    host: &str,
    request: &tauri::http::Request<Vec<u8>>,
) -> Result<tauri::http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let folder = host.to_string();
    let path = folder.clone() + request.uri().path();
    let index_html = folder + "/" + "index.html";

    // Security: prevent directory traversal
    if path.contains("..") || path.contains('\\') || path.contains("_vercel") {
        return Ok(tauri::http::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Invalid path".as_bytes().to_vec())?);
    }

    println!("Loading {}", path);
    // Try to get the file from embedded assets
    if let Some(file) = EMBEDDED_ASSETS.get_file(&path) {
        let body = file.contents().to_vec();
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        return Ok(tauri::http::Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", mime.as_ref())
            .header("Access-Control-Allow-Origin", "*")
            .header("Content-Length", body.len())
            .body(body)?);
    } else if let Some(file) = EMBEDDED_ASSETS.get_file(&index_html) {
        println!("Loading {}", index_html);
        let body = file.contents().to_vec();
        let mime = mime_guess::from_path(index_html).first_or_octet_stream();
        return Ok(tauri::http::Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", mime.as_ref())
            .header("Access-Control-Allow-Origin", "*")
            .header("Content-Length", body.len())
            .body(body)?);
    }

    Ok(tauri::http::Response::builder()
        .header("Content-Type", "text/plain")
        .header("Access-Control-Allow-Origin", "*")
        .status(404)
        .body(b"404 Not Found".to_vec())?)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, test_local_assets])
        // .register_uri_scheme_protocol("qwerty", |_ctx, request| {
        //     handle_local_assets("qwerty", &request).unwrap_or_else(|_| {
        //         tauri::http::Response::builder()
        //             .status(500)
        //             .body(b"Internal Server Error".to_vec())
        //             .unwrap()
        //     })
        // })
        .register_uri_scheme_protocol("typewords", |_ctx, request| {
            handle_local_assets("typewords", &request).unwrap_or_else(|_| {
                tauri::http::Response::builder()
                    .status(500)
                    .body(b"Internal Server Error".to_vec())
                    .unwrap()
            })
        })
        .setup(|_app| {
            // Log the available hosts
            println!("Local assets server configured for hosts:");
            println!("- qwerty.localhost");
            println!("- typewords.localhost");
            println!("Access via: http://qwerty.localhost/ or http://typewords.localhost/");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
