# A Drag-In Web Server (WIP)

A small project you can drag a folder or file into the exe to start a web server hosting the folder.

This project is for learning and practising purpose only. So I will try not to use any third-party libraries (crates).

Currently, things needed to be implemented:
1. TCP connection handling;
2. HTTP header parsing;
3. file reading;
4. url decoding(percent-encoding);
5. MIME types;

TODO:
1. use a new port if the current port is in use
2. MIME types
3. URL decoding
4. long time inactive 
5. concurrency