#!/bin/sh

rustc -O -o bin/table-music-proxy table-music-proxy.rs && ./bin/table-music-proxy > failed-words2.txt

