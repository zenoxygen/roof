[![Build Status](https://gitlab.com/zenoxygen/roof/badges/master/pipeline.svg)](https://gitlab.com/zenoxygen/roof/pipelines)
[![Crates.io](https://img.shields.io/crates/v/roof.svg)](https://crates.io/crates/roof)
[![Docs](https://docs.rs/roof/badge.svg)](https://docs.rs/roof)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

# roof

A minimalist, fast and reliable utility to share files.

## About

Roof has two main modes of functionality. These are the serve mode and the download mode.

The serve mode can be used to serve a file. If a directory is specified, a compressed tar of that directory is served. It allows to customize the IP/port, and how many times this file can be shared. Furthermore, it has an option to offer itself, so the pair can download it and send you something back.

The download mode can be used to obtain a file from a remote pair.

## Usage

```
roof 0.3.0
zenoxygen <zenoxygen@protonmail.com>
A minimalist, fast and reliable utility to share files.

USAGE:
    roof [FLAGS] [OPTIONS] <file>

FLAGS:
    -h, --help       Prints help information
    -s, --serve      When specified, roof will serve itself
    -V, --version    Prints version information

OPTIONS:
    -c, --count <count>        How many times the file/directory will be served [default: 1]
    -i, --ip_addr <ip_addr>    The address to serve the file/directory from [default: 127.0.0.1]
    -p, --port <port>          The port to serve the file/directory from [default: 8080]

ARGS:
    <file>    The file/directory to serve or the URL to download from
```

## Examples

### Serve/download a file

```
$> ./roof myfile.txt
Serving on http://127.0.0.1:8080/myfile.txt

$> ./roof http://127.0.0.1:8080/myfile.txt
Downloading myfile.txt
⠁ 100KB/100KB [#########################################] 100%
```

### Serve/download a directory

```
$> ./roof mydir
Serving on http://127.0.0.1:8080/mydir.tar.gz

$> ./roof http://127.0.0.1:8080/mydir.tar.gz
Downloading mydir.tar.gz
⠁ 100KB/100KB [#########################################] 100%
```

### Serve itself

```
$> ./roof -s
Serving on http://127.0.0.1:8080/roof
```

## Documentation

Learn more about Roof here: [https://docs.rs/roof](https://docs.rs/roof).

## License

Roof is distributed under the terms of the [MIT License](LICENSE).
