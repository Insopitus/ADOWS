# ADOWS

ADOWS is a small program that runs a local http server to host you web page.

## Usage

1. get `adows.exe` from the [release page](https://github.com/Insopitus/ADOWS/releases), or use `cargo build --release` to build it yourself.

2. Drag and drop a folder to `adows.exe`, the server will then start hosting the target directory. You can also put `adows.exe` in the target directory and run it there. Further more, since ADOWS is a CLI program you can also use the command line: `adows.exe your/target/directory`.

3. The server starts at `http://localhost:8080/` by default, if the port `8080` is in use, ADOWS will try to use `8081`, `8082`, until it finds an available port.

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
3. error recovery;
6. logging requester's ip, time requested, etc.

## May Not Do
1. more http header supports, only if it's needed;
8. HTTP/2.0 support
9. gzip and other compressions
