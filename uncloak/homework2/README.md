# Exercises
This week's has a base set of exercises, and an extended exercise set for those who would like more practice.

## Chapter 3 (p. 61)
1; How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?

6; Consider a new block cipher, *DES2*, that consists only of two rounds of the *DES* block cipher. *DES2* has the same block and key size as *DES*. For this question you should consider the *DES* $F$ function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for *DES2* under a single, unknown key. Give an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit *DES* key. Can your algorithm be converted into a distinguishable attack against *DES2*?

8; Using an existing cryptographic library, decrypt the following ciphertext (in hex)
```hex
	53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07
```
with the following 256-bit key (also in hex):
```hex
	80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
	00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
```
using *AES*.

## Chapter 4 (p. 107)
1; Let $P$ be a plaintext and let $\ell(P)$ be the length of $P$ in bytes. Let $b$ be the block size of the block cipher in bytes. Explain why the following is not a good padding scheme:

Determine the minimum number of padding bytes necessary in order to pad the plaintext to a block boundary. This is a number $n$ which satisfies $0 ≤ n ≤ b − 1$ and $n + l(P)$ is a multiple of $b$. Pad the plaintext by appending $n$ bytes, each with value $n$.

3; Suppose you, as an attacker, observe a pair of  32-byte ciphertexts $C, C'$, and you know these ciphertexts were generated using CTR mode with the same nonce. By chance, you obtain $P$ corresponding to $C$. What information, if any, can you infer about the plaintext $P'$ corresponding to $C'$?

4; The ciphertext (in hex):
```hex
87 F3 48 FF 79 B8 11 AF 38 57 D6 71 8E 5F 0F 91
7C 3D 26 F7 73 77 63 5A 5E 43 E9 B5 CC 5D 05 92
6E 26 FF C5 22 0D C7 D4 05 F1 70 86 70 E6 E0 17
```
was generated with the 256-bit AES key (also in hex)
```hex
80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
```
using CBC mode with a random IV. The IV is included at the beginning of the ciphertext. Decrypt this ciphertext. You may use an existing cryptography library for this exercise.

6; Let $P_1$, $P_2$ be a message that is two blocks long, and let $P'_1$ be a message that is one block long. Let $C_0, C_1, C_2$ be the encryption of $P_1, P_2$ using CBC mode with a random IV and a random key, and let $C'_0, C'_1$ be the encryption of $P'_1$ using CBC mode with a random IV and the same key. Suppose an attacker knows $P_1, P_2$ and suppose the attacker intercepted and thus knows $C_0, C_1, C_2$ and $C'_0, C'_1$. Further suppose that, by random chance, $C'_1 = C_2$. Show that the attacker can compute $P'_1$.


# Extended Exercises
## Chapter 3
5; Suppose you have a processor that can perform a single DES encryption or decryption operation in $2^{-26}$ seconds. Suppose you also have a large number of plaintext-ciphertext pairs for $DES$ under a single unknown key. How many hours would it take, on average, to find that $DES$ key, using an exhaustive search approach and a single processor? How many hours would it take, with a collection of $2^{14}$ processors?

9; Using an existing cryptography library, encrypt the following plaintext (in hex)
```ex
	29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D
```
with the following 256-bit key from problem 8, using *AES*. Then re-encrypt and decrypt it using a 3072-bit RSA key with GnuPG, or your choice of asymmetric crypto CLI.

10; Write a program that experimentally demonstrates the complementation property for DES. This program should take as input a key $K$ and a plaintext $P$ and demonstrate that the DES complementation property holds for this key and plaintext. You may use an existing cryptography library for this exercise.

## Chapter 4
5; Encrypt the plaintext
```hex
62 6C 6F 63 6B 20 63 69 70 68 65 72 73 20 20 20
68 61 73 68 20 66 75 6E 63 74 69 6F 6E 73 20 78
62 6C 6F 63 6B 20 63 69 70 68 65 72 73 20 20 20
```
using AES in ECB mode and the key
```hex
80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01.
```
You may use an existing cryptography library for this exercise.

- Implement a pair of functions: A [PKCS 7](https://en.wikipedia.org/wiki/PKCS_7) message padding function, and a padding validation function that takes a message and validates whether it has a correct padding.