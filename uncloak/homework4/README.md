# authenticated encryption and tls

[session 4 notes](https://github.com/thor314/uncloak/blob/main/courses/rust%20cryptography%20engineering/course-2022-12-09%20Session%204%20Notes.md)

**authenticated encryption** is the culmination of what we've learned so far -- symmetric encryption: 
confidentiality (ciphers), authentication, and integrity (hashes and MACs).

**tls** pulls together multiple threads in the progression of our study: 
* it uses authenticated encryption post-handshake
* the handshake phase introduces key exchange and asymmetric encryption, the next phase of the course
* it is a widespread cryptosystem that's used by everyone daily


### resources
* ~~cryptography engineering~~: outdated material
* real world cryptography
  - authenticated encyrption: chapter 4
  - tls: chapter 9
* crypto101
  - tls: chapter 15
* a graduate course in applied cryptography
  - authenticated encryption: chapter 9
  - tls: section 9.8
* additional useful resources:
  - [illustrated tls connection](https://tls13.xargs.org/)
  - [Authenticated Encryption: Relations among Notions
    and Analysis of the Generic Composition Paradigm](https://link.springer.com/content/pdf/10.1007/3-540-44448-3_41.pdf)
  - [Should we MAC-then-encrypt or encrypt-then-MAC?](https://crypto.stackexchange.com/questions/202/should-we-mac-then-encrypt-or-encrypt-then-mac)
  - [The Cryptographic Doom Principle](https://moxie.org/2011/12/13/the-cryptographic-doom-principle.html)

## Exercises
1. Justify or disqualify each of the following schemes, with message $m$, tag $t$, and ciphertext $c$. 
   1. $t=MAC(m)\quad c=E(m)$, send $(c,t)$
      - aka encrypt and mac
      - used by ssh
      - can be secure with the correct cipher, mac and implementation
      - advantages
        - encryption and authentication can happen in parallel
        - plaintext integrity
      - disadvantages
        - no ciphertext integrity
        - weak MAC could leak information about the plaintext

   2. $t = MAC(m)\quad c = E(m||t)$, send $c$
      - aka mac then encrypt
      - used by tls < v1.2, 802.11i
      - can be secure with the correct cipher, mac and implementation 
      - advantages
        - plaintext integrity
        - MAC is protected by a layer of encryption
      - disadvantages
        - no ciphertext integrity
        - if cipher scheme is malleable, MAC and plaintext can be modified
    
   3. $c=E(m)\qquad t=MAC(c)$, send $(c,t)$
      - aka encrypt then mac
      - used by ipsec and tls >= v1.2
      - the best approach
      - advantages
        - ciphertext integrity
        - strong MAC can cover for vulnerable cipher
        - computationally efficient because data can be dropped before decryption if MAC is invalid
      - disadvantages
        - violates Horton Principle

2. You're the adversary, watching a TLS handshake. Pick three steps from [TLS Handshake - OSDev Wiki](https://wiki.osdev.org/TLS_Handshake#Handshake_Overview), and describe how the step prevents you from (pick one):
   1. reading message content (confidentiality)
       - client and server exchange keys to establish a shared secret for encrypting traffic 
   2. tampering with message content (integrity)
       - at the end of the handshake, parties exchange an authentication tag of their exchanges to confirm that there was no tampering of their communication
   3. impersonating either party (authenticity)
       - server sends its certificate and client confirms that it's signed by a CA 
