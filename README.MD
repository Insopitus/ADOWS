# ADOWS, A Drag-Onto Web Server.

## Description

ADOWS is a small program that you can drag-drop your static webpage file folder to it to start a web server that hosts your webpage.

## Usage

Drag a folder that contains a `index.html` file at its root onto the `drag_in_web_server.exe` file in the release page.

## Misc

This project is for learning and practising purpose mainly, which means I will try not to use any third-party libraries (crates).

As a result, the server could be cranky and would never go anywhere near production standard.

features implemented:
1. TCP connection handling;
2. HTTP header parsing;
3. file reading;

## Todos:
1. use a new port if the current port is in use
2. MIME types (es module loading needs strict MIME type)
3. URL decoding(non ASCII file names)
5. concurrency
6. error recovery
7. cache
8. relative path (../)
