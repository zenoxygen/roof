// Copyright (c) 2020 zenoxygen
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::{anyhow, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

static NOT_FOUND: &[u8] = b"Not Found";
static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";

/// Make HTTP 404 response (Not Found).
fn make_not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOT_FOUND.into())
        .unwrap()
}

/// Make HTTP 500 response (Internal Server Error).
fn make_internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(INTERNAL_SERVER_ERROR.into())
        .unwrap()
}

/// Make HTTP 200 response (Status Ok).
fn make_status_ok(buf: Vec<u8>, len: u64) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_LENGTH, len)
        .body(buf.into())
        .unwrap()
}

/// Make file name.
fn make_file_name(file_path: &PathBuf) -> String {
    let file_name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();
    file_name.replace(|c: char| c.is_whitespace(), "_")
}

/// Send file.
async fn send_file(file_path: PathBuf) -> Result<Response<Body>> {
    // Read file entirely into memory
    if let Ok(mut file) = File::open(&file_path).await {
        let mut buf = Vec::new();
        if file.read_to_end(&mut buf).await.is_ok() {
            // Get file metadata
            let meta = file.metadata().await?;
            let len = meta.len();
            // Display status
            let file_name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();
            println!("Sending {}", file_name);
            // Send file
            return Ok(make_status_ok(buf, len));
        }
        // Return internal server error
        return Ok(make_internal_server_error());
    }

    // Return not found
    Ok(make_not_found())
}

/// Handle request.
async fn handle_request(req: Request<Body>, file_path: PathBuf) -> Result<Response<Body>> {
    // Make file name
    let file_name = make_file_name(&file_path);
    // Check if requested file match with served file
    match (req.method(), req.uri().path().trim_start_matches('/')) {
        (&Method::GET, name) if name == file_name => send_file(file_path).await,
        _ => Ok(make_not_found()),
    }
}

/// Compress directory.
fn compress_dir(dir_path: &PathBuf, tarball_path: &str) -> Result<(), std::io::Error> {
    // Create tarball
    let tar_gz = std::fs::File::create(tarball_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tarball = tar::Builder::new(enc);
    // Add directory content into tarball
    tarball.append_dir_all(dir_path, dir_path)?;

    Ok(())
}

/// Serve file.
#[tokio::main]
pub async fn serve_file(
    mut file_path: PathBuf,
    ip_addr: &str,
    port: &str,
    count: usize,
) -> Result<()> {
    // Create address to bind to server
    let ip_port = format!("{}:{}", ip_addr = ip_addr, port = port);
    let addr = match ip_port.parse() {
        Ok(addr) => addr,
        Err(_) => return Err(anyhow!("could not parse IP/port configuration")),
    };

    // Create counter
    let counter = Arc::new(AtomicUsize::new(count));

    // Compress directory into tarball
    if file_path.is_dir() {
        let mut tarball_path = file_path.clone();
        tarball_path.set_extension("tar.gz");
        if compress_dir(&file_path, tarball_path.as_path().to_str().unwrap()).is_err() {
            return Err(anyhow!("could not compress directory into tarball"));
        }
        file_path = tarball_path;
    }

    // Make file name
    let file_name = make_file_name(&file_path);

    // Clone file path
    let file_path = file_path.clone();

    // The closure inside `make_service_fn` is run for each connection,
    // creating service to handle requests for that specific connection
    let service = make_service_fn(move |_| {
        // Each connection could send multiple requests,
        // service needs a clone to handle later requests
        let file_path = file_path.clone();
        let counter = counter.clone();

        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                // Decrement counter, return previous value
                let count = counter.fetch_sub(1, Ordering::AcqRel);
                // Exit success when reached maximum number of requests
                if count == 0 {
                    std::process::exit(0);
                }
                // Handle incoming request
                handle_request(req, file_path.clone())
            }))
        }
    });

    // Create server bound on provided address
    let server = match Server::try_bind(&addr) {
        Ok(server) => server,
        Err(_) => return Err(anyhow!("could not bind server to address provided")),
    };

    // Display status
    println!("Serving on http://{}:{}/{}", ip_addr, port, file_name);

    // Wait for server to complete serving
    if server.serve(service).await.is_err() {
        return Err(anyhow!("server failed while serving file"));
    }

    Ok(())
}
