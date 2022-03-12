

## Finding frequency of each letter in each position:


```sh
cut -c1-1 dictionary.txt | sort | uniq -c > dictionary-pos1-letter-frequency.txt
cut -c2-2 dictionary.txt | sort | uniq -c > dictionary-pos2-letter-frequency.txt
cut -c3-3 dictionary.txt | sort | uniq -c > dictionary-pos3-letter-frequency.txt
cut -c4-4 dictionary.txt | sort | uniq -c > dictionary-pos4-letter-frequency.txt
cut -c5-5 dictionary.txt | sort | uniq -c > dictionary-pos5-letter-frequency.txt
```

![Letter Frequency Per Position](letters-frequency-per-position.png)

## Running

```sh
rustc w1.rs -o bin/w1 && time ./bin/w1 > failed-words.txt
```

## Compiling with optimization -> Huge difference!!!


```sh
rustc -O w1.rs -o bin/w1_optimized_level_2
```

```sh
$ time ./bin/w1 > /dev/null

real	0m21,125s
user	0m20,794s
sys	0m0,120s

$ time ./bin/w1_optimized_level_2 > /dev/null

real	0m0,785s
user	0m0,720s
sys	0m0,064s
```
