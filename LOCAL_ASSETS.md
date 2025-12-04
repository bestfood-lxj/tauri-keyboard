# Local Assets Server Documentation

This Tauri application includes a custom local assets server that can serve static files from two different hosts.

## Overview

The local assets server is implemented using Tauri's custom URI scheme protocol feature. It registers a `local-assets://` protocol that can serve different assets based on the requested host.

## Supported Hosts

The server supports two hosts:

1. **localhost:3000** - `local-assets://localhost:3000/`
2. **127.0.0.1:3000** - `local-assets://127.0.0.1:3000/`

Each host serves the same set of assets but can be configured independently if needed.

## Available Assets

Both hosts serve the following assets:

- `/` or `/index.html` - HTML page listing available assets
- `/favicon.ico` - Application icon (32x32.png)
- `/logo.png` - Application logo (128x128.png)
- `/config.json` - JSON configuration file with host information

## Usage

### From Frontend JavaScript

You can request local assets using the standard `fetch` API:

```javascript
// Request from localhost host
const response = await fetch('local-assets://localhost:3000/config.json');
const data = await response.json();

// Request from IP host
const logoResponse = await fetch('local-assets://127.0.0.1:3000/logo.png');
const logoBlob = await logoResponse.blob();
```

### Using Tauri Commands

The application also includes a Tauri command for testing the local assets:

```javascript
const { invoke } = window.__TAURI__.core;

// Test all hosts and assets
const results = await invoke('test_local_assets');
console.log('Asset test results:', results);
```

## Testing

The application includes a testing interface in the main HTML file with three test modes:

1. **Test localhost:3000** - Tests all assets on the localhost host
2. **Test 127.0.0.1:3000** - Tests all assets on the IP host  
3. **Test via Rust Command** - Uses the Tauri command to get asset information

## Implementation Details

### Rust Side (`lib.rs`)

- `handle_local_assets()` - Main handler function for processing requests
- Asset data is embedded at compile time using `include_bytes!` macro
- Different hosts can serve different content if needed
- Returns appropriate HTTP headers including CORS headers

### Configuration (`tauri.conf.json`)

- Enables the `assetProtocol` feature in security settings
- Requires the `protocol-asset` feature in Cargo.toml

### Frontend Integration

- Uses standard web APIs (fetch) to request assets
- Includes fallback error handling
- Provides visual feedback for testing

## Error Handling

- Returns 404 for unknown paths
- Returns 500 for server errors
- Includes CORS headers for cross-origin requests
- Provides detailed error messages in development

## Security Considerations

- Assets are embedded at compile time (no file system access)
- CORS headers allow cross-origin requests
- Only predefined assets are served
- No dynamic file serving or directory traversal

## Extending

To add new assets:

1. Add the asset file to the project
2. Update the `host_assets` HashMap in `handle_local_assets()`
3. Use `include_bytes!` to embed the asset
4. Rebuild the application

To add new hosts:

1. Create a new HashMap for the host's assets
2. Add it to the `host_assets` HashMap
3. Update documentation and tests as needed

## Development

To run the application in development mode:

```bash
cd src-tauri
cargo tauri dev
```

The local assets server will be automatically configured and available at the registered protocol URLs.