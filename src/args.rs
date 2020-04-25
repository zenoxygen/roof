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

extern crate clap;

use clap::{App, Arg};

/// Parse arguments.
pub fn parse_args<'a>() -> clap::ArgMatches<'a> {
    let args_conflicts = ["file"];
    App::new("roof")
        .version("0.3.0")
        .about("A minimalist, fast and reliable utility to share files.")
        .author("zenoxygen <zenoxygen@protonmail.com>")
        .arg(
            Arg::with_name("file")
                .help("The file/directory to serve or the URL to download from")
                .required_unless("serve")
                .number_of_values(1),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("How many times the file/directory will be served")
                .number_of_values(1)
                .default_value("1"),
        )
        .arg(
            Arg::with_name("ip_addr")
                .short("i")
                .long("ip_addr")
                .help("The address to serve the file/directory from")
                .number_of_values(1)
                .default_value("127.0.0.1"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("The port to serve the file/directory from")
                .number_of_values(1)
                .default_value("8080"),
        )
        .arg(
            Arg::with_name("serve")
                .short("s")
                .long("serve")
                .help("When specified, roof will serve itself")
                .conflicts_with_all(&args_conflicts)
                .empty_values(true),
        )
        .get_matches()
}
