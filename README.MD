# ADOWS, A Drag-Onto Web Server.

ADOWS is a small program that you can drag-drop your static webpage file folder to it to start a web server that hosts your webpage.

## Usage

1. get `adows.exe` from the [release page](https://github.com/Insopitus/ADOWS/releases), or use `cargo build --release` to build it yourself.

2. Drag and drop a folder to `adows.exe`, the server will then start hosting the target directory. You can also put `adows.exe` in the target directory and run it there. Further more, since ADOWS is a CLI program you can also use the command line: `adows.exe your/target/directory`.

3. The server starts at `http://localhost:8080/` by default, if the port `8080` is in use, ADOWS will try to use `8081`, `8082`, until it finds an available port.

*ADOWS should work on linux and macOS but I didn't test it. You also need to build the executable youself in those cases.*

## Description

This project is based on the official rust book's "build a web server" chapter. The original one is very rough, unstable and lacks a lot of features. My goal is to add more funtionality to it and make it robust. 

On the other hand, the project is written for learning and practising purpose. I avoid using third-party libraries/crates to get a better understanding of how things work. As a result, ADOWS may not be production-ready at any moment, but it should be enough for general use case.

For now, ADOWS has meet its goal for common usage. It's fast, stable, memory safe and uses little RAM (a few megabytes).

**Features:**
1. TCP connection handling;
1. HTTP header parsing: not fully featured, just enough for a webpage host server;
1. MIME type support for common media types;
1. URI decoding to support non-ASCII directory/file names;
1. file chunk reading so the server won't use too many RAM while hosting large files;
1. concurency;

**Todos:**
1. implement keep-alive header to boost performance
1. error recovery;
1. caching;
1. more http header supports;
1. to use more reference(lifetime), less clones.
1. logging requester's ip, time requested, etc.
1. security: don't know if it's nessassary.
1. HTTP/2.0 support, maybe?
1. gzip: may need external crates.
