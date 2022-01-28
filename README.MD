# ADOWS, A Drag-Onto Web Server.

ADOWS is a small program that you can drag-drop your static webpage file folder to it to start a web server that hosts your webpage.

## Usage

1. get `adows.exe` from the [release page](https://github.com/Insopitus/ADOWS/releases), or use `cargo build --release` to build it yourself.

2. Drag and drop a folder to `adows.exe`, the server will then start hosting the target directory. You can also put `adows.exe` in the target directory and run it there. Further more, since ADOWS is a CLI program you can also use the command line: `adows.exe your/target/directory`.

3. The server starts at `http://localhost:8080/` by default, if the port `8080` is in use, ADOWS will try to use `8081`, `8082`, until it finds a usable port.

*ADOWS should work on linux and macOS but I didn't test it. You also need to build the executable youself.*

## Description

This project is based on the official rust book's "build a web server" chapter. The original one is very rough, unstable and lacks a lot of features. My goal is to add more funtionality to it and make it robust. 

On the other hand, the project is written for learning and practising purpose. I avoid using third-party libraries/crates. As a result, ADOWS may not be production-ready at any moment, but it should be enough for general use case.

For now, ADOWS has meet the goal for general usage. It's fast, stable, memory safe and uses little RAM (a few megabytes).

Features implemented:
1. TCP connection handling;
2. HTTP header parsing;
4. MIME type support for common media types;
5. URI decoding;
6. file chunk reading;
7. concurency;

Todos:
1. implement keep-alive header to boost performance
2. error recovery;
3. caching;
4. more http header supports;
5. to use more reference(lifetime), less clones.
8. logging requester's ip, time requested, etc.
9. security: (DoS is already handled)
10. HTTP/2.0 support, maybe?
11. gzip: may need external crates.
