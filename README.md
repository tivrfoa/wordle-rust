

Finding frequency of each letter in each position:


```sh

cut -c1-1 dictionary.txt | sort | uniq -c > dictionary-pos1-letter-frequency.txt
cut -c2-2 dictionary.txt | sort | uniq -c > dictionary-pos2-letter-frequency.txt
cut -c3-3 dictionary.txt | sort | uniq -c > dictionary-pos3-letter-frequency.txt
cut -c4-4 dictionary.txt | sort | uniq -c > dictionary-pos4-letter-frequency.txt
cut -c5-5 dictionary.txt | sort | uniq -c > dictionary-pos5-letter-frequency.txt
```
