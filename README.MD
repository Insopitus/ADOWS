# ADOWS, A Drag-Onto Web Server.


ADOWS is a small program that you can drag-drop your static webpage file folder to it to start a web server that hosts your webpage.

## Usage

Drag a file folder that contains a `index.html` file right inside it onto the `adows.exe` file which you can get from the [release page](https://github.com/Insopitus/ADOWS/releases).
You can also put `adows.exe` in the target directory and run it to host the directory it's in.

## Description

This project is based on the official rust book's "build a web server" chapter. The original one is very rough, unstable and lacks a lot of features. My goal is to add more funtionality to it and make it robust. 

On the other hand, the project is written for learning and practising purpose mainly. I will avoid using third-party libraries/crates. As a result, ADOWS may not be production-ready at any moment. But it's enough for general use case.

Right now, ADOWS is ready to use. It's fast, stable, memory safe and uses little RAM (a few megabytes).

Features implemented:
1. TCP connection handling;
2. HTTP header parsing;
3. file reading;
4. MIME type support for main media types;
5. URI decoding;
6. file read stream chunking
7. concurency

Todos:
1. implement keep-alive header to boost performance
2. error recovery;
3. caching;
4. more http header supporting;
5. to use more reference(lifetime), less clones.
7. ~~memory usage: don't read the full file at once, (read part of it and write to tcp stream then repeat.. `chuncking` as they say.)~~ read chunking data to heap memory (since stack memory is limited)
8. logging requester's ip, time requested, etc.
9. security: (DoS is already handled)
10. HTTP/2.0 support, maybe?
11. gzip: may need external crates.
