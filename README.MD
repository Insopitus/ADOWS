# ADOWS, A Drag-Onto Web Server.

## Description

ADOWS is a small program that you can drag-drop your static webpage file folder to it to start a web server that hosts your webpage.

## Usage

Drag a folder that contains a `index.html` file at its root onto the `adows.exe` file in the release page.
You can also put `adows.exe` in the target directory and run it to host the directory it's in.

## Limitations

1. the directroy name and file names should not contain non-ASCII characters, or ADOWS cannot find the target file. It will be fixed when I finish the URL decoding function.

## Misc

This project is for learning and practising purpose mainly, which means I will try not to use any third-party libraries (crates).

As a result, the server could be cranky and would never go anywhere near production standard.

features implemented:
1. TCP connection handling;
2. HTTP header parsing;
3. file reading;
4. MIME type support for main media types;
5. URI decode;

## Todos:
5. concurrency
6. error recovery
7. cache
8. relative path (../)
