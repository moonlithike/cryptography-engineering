# authenticated encryption and tls

[session 4 notes](https://github.com/thor314/uncloak/blob/main/courses/rust%20cryptography%20engineering/course-2022-12-09%20Session%204%20Notes.md)

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

## Exercises
1. Justify or disqualify each of the following schemes, with message $m$, tag $t$, and ciphertext $c$. 
   1. $t=MAC(m)\quad c=E(m)$, send $(c,t)$
   2. $t = MAC(m)\quad c = E(m||t)$, send $c$
   3. $c=E(m)\qquad t=MAC(c)$, send $(c,t)$


2. You're the adversary, watching a TLS handshake. Pick three steps from [TLS Handshake - OSDev Wiki](https://wiki.osdev.org/TLS_Handshake#Handshake_Overview), and describe how the step prevents you from (pick one):
   1. reading message content (confidentiality)
   2. tampering with message content (integrity)
   3. impersonating either party (authenticity)