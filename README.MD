# ADOWS, A Drag-Onto Web Server.


ADOWS is a small program that you can drag-drop your static webpage file folder to it to start a web server that hosts your webpage.

## Usage

Drag a file folder that contains a `index.html` file right inside it onto the `adows.exe` file which you can get from the [release page](https://github.com/Insopitus/ADOWS/releases).
You can also put `adows.exe` in the target directory and run it to host the directory it's in.

## Limitations

1. The server is single-threaded right now. If it's used to host large files, the process would be slow.

## Description

This project is inspired by the official rust book's "write a web server" part. The original one is very rough, unstable and lacks a lot of features. So I tried to add more funtionality and make it robust. 

On the other hand, the project is written for learning and practising purpose mainly. I try not to use any third-party libraries/crates (reinvented many wheels).

ADOWS is full-functional now. All I'll do later is optimizing. The program is usable for the moment, but I dare not say it's production-ready.

Features implemented:
1. TCP connection handling;
2. HTTP header parsing;
3. file reading;
4. MIME type support for main media types;
5. URI decoding;

Todos:
1. concurrency;
2. error recovery;
3. caching;
5. to use more reference(lifetime), less clones.
6. memory usage: the program uses *exactly* twice as much memory as the hosted file (sometimes)
7. memory usage: don't read the full file at once, (read part of it and write to tcp stream then repeat.. `chuncking` as they say.)
8. logging requester's ip, time requested, etc.
9. security: (DoS is already handled)
10. HTTP/2.0 support, maybe?
