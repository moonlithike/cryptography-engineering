# introduction

[session 1 notes](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-11-18+Session+1+Notes)

### resources
* cryptography engineering, chapter 1 & 2


### exercises
Ch 1:
- Q10. Describe a concrete example where improving the security of a system against one type of 
attack can increase the likelihood of other attacks.
  - using a password manager enables users to create and save high entropy passwords for their accounts,
    but if an attacker takes control of the password manager itself they will have access to all the user's 
    credentials


Ch 2:
- Q3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. 
      How many keys need to be exchanged in total.
  - 29 + 28 + ... + 1 = n(n-1)/2 = 435
 
- Q4. Suppose Bob receives a message signed using a digital signature scheme with Alice's secret signing key. 
      Does it prove that Alice saw the message and chose to sign.
  - no, only that something that controlled her signing key was able to sign the message
 
- Q6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. 
      Does this mean the encryption scheme is secure?
  - no, without the decryption key the attacker could still find information about the plaintext (eg size, patterns)
 
- Q7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of
      all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.
  - 256

General:
- Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help:
    - https://cryptography.rs/
    - https://lib.rs/ (librs is equivalent to crates.io, with a different interface)
- Benchmark the speed of an algorithm in the two different implementations with [Criterion](https://lib.rs/crates/criterion).
    - User guide: https://bheisler.github.io/criterion.rs/book/index.html
    - Video intro: https://youtu.be/eIB3Pd5LBkc
- Implement the [Vignère cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) in 100 lines or less.
- What is a side channel attack? Is your cipher implementation constant time?