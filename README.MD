# ADOWS

ADOWS is a small program that runs a local http server to host you web page.

## Usage

1. Get `adows.exe` from the [release page](https://github.com/Insopitus/ADOWS/releases), or use `cargo build --release` to build it yourself.

2. Run adows in the terminal. Or, you can copy adows.exe to your target directory and double click to run it there.
    ```
    Usage:
        adows [OPTIONS] [DIRECTORY] [PORT]

    Options:
        -v, --version       print the current version of adows and exit.
        -h, --help          print this message and exit.
        -c, --cross-origin  send cross-origin header field.
        -b, --open-browser  open the browser on server start.
    ```
    example: 

    run `adows -bc ./www` to host your files in directory `./www` in [http://localhost:8080](http://localhost:8080), using the default port 8080, and open the browser once the server starts.


3. If the port is in use or is not available, adows will try to use another usable port, like 8081, 8082.

*for linux users, you have to run `cargo build --release` to build it on your machine and run adows in the terminal.

## Description

This project is based on the official rust book's "build a web server" chapter. The original one is very rough, unstable and lacks a lot of features. My goal is to add more funtionality to it and make it robust. 

On the other hand, the project is written for learning and practising purpose. I avoid using third-party libraries/crates so I can get a better understanding of how things work, which also makes the executable minimal. As a result, adows is not feature rich but meets the basic needs for a static file http server.

## Features:
1. TCP connection handling;
1. HTTP header parsing: not fully featured, but enough for static webpages;
1. MIME type support for common media types;
1. URI decoding to support non-ASCII directory/file names, (they still have to be valid unicode though);
1. file chunk reading so the server won't use too many RAM while hosting large files;
1. concurency;
1. caching;

## Todos:
1. implement keep-alive header to boost performance
1. fewer allocations (mostly strings)
3. multi-thread error recovery;
4. async (if implementing a async runtime is not that complicated);
5. HTTP/2

## May Not Do
1. more http header supports, only if it's needed;
9. gzip and other compressions
