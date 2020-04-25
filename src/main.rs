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

extern crate anyhow;
extern crate clap;
extern crate flate2;
extern crate hyper;
extern crate indicatif;
extern crate tar;
extern crate tokio;
extern crate url;

mod args;
mod client;
mod server;

use crate::args::parse_args;
use crate::client::download_file;
use crate::server::serve_file;

use anyhow::{anyhow, Result};
use url::Url;

use std::path::{Path, PathBuf};

/// Parse count.
fn parse_count(count: &str) -> usize {
    match count.parse::<usize>() {
        Ok(valid_count) => valid_count,
        Err(_) => 1,
    }
}

/// Get executable path.
fn get_exe_path() -> Result<PathBuf> {
    match std::env::current_exe() {
        Ok(exe_path) => Ok(exe_path),
        Err(_) => Err(anyhow!("could not access executable path")),
    }
}

/// Run program.
fn run(args: clap::ArgMatches) -> Result<()> {
    let ip_addr = args.value_of("ip_addr").unwrap();
    let port = args.value_of("port").unwrap();
    let count = parse_count(args.value_of("count").unwrap());

    // Serve or download file
    if args.is_present("file") {
        let file = args.value_of("file").unwrap();
        if Path::new(file).exists() {
            let file_path = PathBuf::from(file);
            serve_file(file_path, &ip_addr, &port, count)?;
        } else if Url::parse(file).is_ok() {
            download_file(file)?;
        } else {
            return Err(anyhow!(
                "unable to find a file/directory to serve or an URL to download from"
            ));
        }
    // Serve itself
    } else {
        let exe_path = get_exe_path()?;
        serve_file(exe_path, &ip_addr, &port, count)?;
    }

    Ok(())
}

fn main() {
    // Parse arguments
    let args = parse_args();

    // Run program, eventually exit failure
    if let Err(error) = run(args) {
        println!("Error: {}", error);
        std::process::exit(1);
    }

    // Exit success
    std::process::exit(0);
}
