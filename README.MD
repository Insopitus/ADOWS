# ADOWS, A Drag-Onto Web Server.


ADOWS is a small program that you can drag-drop your static webpage file folder to it to start a web server that hosts your webpage.

## Usage

Drag a folder that contains a `index.html` file at its root onto the `adows.exe` file in the release page.
You can also put `adows.exe` in the target directory and run it to host the directory it's in.

## Limitations

1. The server is single-threaded right now. If it's used to host large files, the process would be slow.

## Description

This project is inspired by the official rust book's "write a web server" part. The original one is very rough, unstable and lacks a lot of features. So I tried to add more funtionality and make it robust. 

On the other hand, the project is written for learning and practising purpose mainly. I try not to use any third-party libraries/crates (reinvented many wheels).

The program is full-functional now. All I'll do later is optimizing. The program is usable for the moment, but I dare not say it's production-ready.

Features implemented:
1. TCP connection handling;
2. HTTP header parsing;
3. file reading;
4. MIME type support for main media types;
5. URI decoding;

Todos:
5. concurrency;
6. error recovery;
7. cache;
8. relative path (../);
