# Bitflipper


This is a very simple program that takes input strings and outputs a list of
strings generated by flipping each byte in the input string.


```sh
$ cargo run -- "amazon.com"
```

outputs

```
0: '`mazon.com'
1: 'cmazon.com'
2: 'emazon.com'
3: 'imazon.com'
4: 'qmazon.com'
5: 'Amazon.com'
6: '!mazon.com'
8: 'alazon.com'
9: 'aoazon.com'
10: 'aiazon.com'
11: 'aeazon.com'
12: 'a}azon.com'
```

... and much more.


Output is guaranteed to be valid utf-8.


Inspired by [DEFCON 19: Bit-squatting: DNS Hijacking Without Exploitation](https://www.youtube.com/watch?v=aT7mnSstKGs)


