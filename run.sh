#!/bin/sh

rustc -O -o bin/w1_optimized_level_2 w1.rs && ./bin/w1_optimized_level_2 > failed-words.txt

