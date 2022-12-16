# hash functions and macs

### resources
* cryptography engineering, chapter 5 & 6
* crypto101, chapter 10 & 11
* real world cryptography, chapter 2 & 3


### exercises
## ch5 exercises
- Exercise 5.3 Consider SHA-512-n, a hash function that first runs SHA-512 and then outputs only the first $n$ bits of the result. Write a program that uses a birthday attack to find and output a collision on SHA-512-n, where n is a multiple of 8 between 8 and 48. Your program may use an existing cryptography library. Time how long your program takes when n is 16, averaged over five runs for each $n.$ How long would you expect your program to take for SHA-512-256? For SHA-512?
- Exercise 5.4 Let SHA-512-n be as in the previous exercise. Write a program that finds a message M that hashes to the following value under SHA-512-16 (in hex):  `3D 4B`. How many tries would you expect the algorithm to need? Running the algorithm 5 times, How many tries did it take on average?
- With command line tools or Criterion, benchmark the [blake3 hash](https://docs.rs/blake3/latest/blake3/) (default is 256 bit output), and compare it to benches of [SHA3-256](https://docs.rs/sha3/latest/sha3/) and [SHA-256](https://docs.rs/sha2/latest/sha2/) (when written without a number, SHA is assumed to be SHA2).

## ch 6 exercises
- Exercise 6.3 Suppose a and b are both one block long, and suppose the sender MACs a, b, and $a || b$ with CBC-MAC. An attacker who intercepts the MAC tags for these messages can now forge the MAC for the message $m=b || (M(b) ⊕ M(a) ⊕ b)$, which the sender never sent. The forged tag for this message is equal to $M(a || b)$, the tag for $a || b$. Justify mathematically why this is true.
- Exercise 6.4 Suppose message $a$ is one block long. Suppose that an attacker has received the MAC $t$ for a using CBC-MAC under some random key unknown to the attacker. Explain how to forge the MAC for a two-block message of your choice. What is the two-block message that you chose? What is the tag that you chose? Why is your chosen tag a valid tag for your two-block message?
- Exercise 6.5 Using an existing cryptography library, compute the MAC of the message:
```hex
4D 41 43 73 20 61 72 65 20 76 65 72 79 20 75 73 65 66 75 6C 20 69 6E 20 63 72 79 70 74 6F 67 72 61 70 68 79 21 20 20 20 20 20 20 20 20 20 20 20
```

using CBC-MAC with AES and the 256-bit key:
```hex
80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
```

- For message authentication, when would you use TupleHash? ParallelHash? KMAC?