use std::collections::HashMap;
use tauri::AppHandle;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn test_local_assets() -> Result<serde_json::Value, String> {
    use std::collections::HashMap;

    let mut results = HashMap::new();

    // Test myapp.localhost
    let localhost_tests = vec![
        ("index", "http://myapp.localhost/"),
        ("config", "http://myapp.localhost/config.json"),
        ("favicon", "http://myapp.localhost/favicon.ico"),
        ("logo", "http://myapp.localhost/logo.png"),
    ];

    let mut localhost_results = HashMap::new();
    for (name, url) in localhost_tests {
        localhost_results.insert(name.to_string(), format!("URL: {}", url));
    }
    results.insert("myapp.localhost".to_string(), localhost_results);

    // Test 127.0.0.1:3000
    let ip_tests = vec![
        ("index", "myapp://127.0.0.1:3000/"),
        ("config", "myapp://127.0.0.1:3000/config.json"),
        ("favicon", "myapp://127.0.0.1:3000/favicon.ico"),
        ("logo", "myapp://127.0.0.1:3000/logo.png"),
    ];

    let mut ip_results = HashMap::new();
    for (name, url) in ip_tests {
        ip_results.insert(name.to_string(), format!("URL: {}", url));
    }
    results.insert("127.0.0.1:3000".to_string(), ip_results);

    Ok(serde_json::to_value(results).unwrap())
}

fn handle_local_assets(
    _app: &AppHandle,
    request: &tauri::http::Request<Vec<u8>>,
) -> Result<tauri::http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let host = request.uri().host().unwrap_or("");
    let path = request.uri().path();

    // Define assets for different hosts
    let mut host_assets: HashMap<&str, HashMap<&str, (&str, Vec<u8>)>> = HashMap::new();

    // Host 1: myapp.localhost
    let mut host1_assets = HashMap::new();
    host1_assets.insert(
        "/favicon.ico",
        (
            "image/x-icon",
            include_bytes!("../icons/32x32.png").to_vec(),
        ),
    );
    host1_assets.insert(
        "/logo.png",
        ("image/png", include_bytes!("../icons/128x128.png").to_vec()),
    );
    host1_assets.insert(
        "/config.json",
        (
            "application/json",
            br#"{"host": "myapp.localhost", "version": "1.0"}"#.to_vec(),
        ),
    );

    // Host 2: 127.0.0.1:3000
    let mut host2_assets = HashMap::new();
    host2_assets.insert(
        "/favicon.ico",
        (
            "image/x-icon",
            include_bytes!("../icons/32x32.png").to_vec(),
        ),
    );
    host2_assets.insert(
        "/logo.png",
        ("image/png", include_bytes!("../icons/128x128.png").to_vec()),
    );
    host2_assets.insert(
        "/config.json",
        (
            "application/json",
            br#"{"host": "127.0.0.1:3000", "version": "1.0"}"#.to_vec(),
        ),
    );

    host_assets.insert("myapp.localhost", host1_assets);
    host_assets.insert("127.0.0.1:3000", host2_assets);

    // Serve assets based on host
    if let Some(assets) = host_assets.get(host) {
        if let Some((content_type, content)) = assets.get(path) {
            return Ok(tauri::http::Response::builder()
                .header("Content-Type", *content_type)
                .header("Access-Control-Allow-Origin", "*")
                .header("Cache-Control", "public, max-age=3600")
                .status(200)
                .body(content.clone())?);
        }
    }

    // Default response for unknown paths
    let default_content = match path {
        "/" | "/index.html" => {
            let html = format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <title>Local Assets - {}</title>
    <link rel="icon" href="/favicon.ico">
</head>
<body>
    <h1>Local Assets Server</h1>
    <p>Host: {}</p>
    <p>Available assets:</p>
    <ul>
        <li><a href="/favicon.ico">/favicon.ico</a></li>
        <li><a href="/logo.png">/logo.png</a></li>
        <li><a href="/config.json">/config.json</a></li>
    </ul>
</body>
</html>"#,
                host, host
            );
            ("text/html", html.into_bytes())
        }
        _ => ("text/plain", b"404 Not Found".to_vec()),
    };

    Ok(tauri::http::Response::builder()
        .header("Content-Type", default_content.0)
        .header("Access-Control-Allow-Origin", "*")
        .status(if path == "/" || path == "/index.html" {
            200
        } else {
            404
        })
        .body(default_content.1)?)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, test_local_assets])
        .register_uri_scheme_protocol("myapp", |ctx, request| {
            handle_local_assets(ctx.app_handle(), &request).unwrap_or_else(|_| {
                tauri::http::Response::builder()
                    .status(500)
                    .body(b"Internal Server Error".to_vec())
                    .unwrap()
            })
        })
        .setup(|_app| {
            // Log the available hosts
            println!("Local assets server configured for hosts:");
            println!("- myapp.localhost");
            println!("- 127.0.0.1:3000");
            println!(
                "Access via: http://myapp.localhost/ or myapp://127.0.0.1:3000/"
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
