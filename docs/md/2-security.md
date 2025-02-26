# Security

## Cryptography

At a high level, subscriptions are encrypted by authenticated encryption using a device-specific key. We aim to prevent retargeting or forging for a new device even in the event that one device is completely compromised. Additionally, subscriptions cannot be retargeted to a new channel even in the case of a compromised device key as every channel has a channel-specific key. On the other hand, frames are encrypted using a frame-specific key to ensure that retargeted frames are not even decrypted into their correct plaintext. 

In order to prevent key reuse, we use a key derivation function (KDF) to derive unique keys for Ascon in various contexts. For our KDF, we use the Keccak Message Authentiation Code (KMAC) construction as defined in [NIST SP800-185](https://doi.org/10.6028/NIST.SP.800-185).

## Primitives

$\mathsf{Ascon128_{Enc}}(K, P, D, N) \rightarrow (C, T)$  
$\mathsf{Ascon128_{Dec}}(K, C, T, D, N) \rightarrow (P)$  
Ascon-128 is an authenticated encryption scheme that takes a key $K$, a plaintext message $P$, associated data $D$, and a public nonce $N$ as input. It outputs a ciphertext $C$ and an authentication tag $T$. For decryption, it takes a key $K$, a ciphertext $C$, authentication tag $T$, associated data $D$, and a public nonce $N$ as input. It outputs the plaintext message $P$.

$\mathsf{KMAC}(K, X, L, S)$  
KMAC is Keccak's keyed message authentication code that takes a key $K$, a message $X$, and a domain separator $S$ as input. It outputs a tag of length $L$.

## Symbols

**General**

$D$ - Decoder ID (4B)  
$C$ - Channel ID (4B)  
$T_{start}$ - Start timestamp (8B)  
$T_{end}$ - End timestamp (8B)  
$T_{frame}$ - Frame timestamp (8B)  

**Deployment Secrets**

$S_{base\_sub}$ - Base subscription secret (32B)  
$S_{base\_chan}$ - Base channel secret (32B)  
$K_{frame}$ - Frame key (16B)  

**Encoder/Decoder Secrets**

$K_{sub}$ - Subscription key (16B), unique to decoder  
$S_{chan}$ - Channel secret (32B), unique to channel  
$K_{pic}$ - Picture key (16B), unique to channel and timestamp  

**Key Derivations**

$K_{sub} = \mathsf{KMAC128}(S_{base\_sub}, D, \mathsf{16B}, \mathsf{"derive\_subscription\_key"})$  
$S_{chan} = \mathsf{KMAC256}(S_{base\_chan}, C, \mathsf{32B}, \mathsf{"derive\_channel\_secret"})$  
$K_{pic} = \mathsf{KMAC128}(S_{chan}, (T_{frame} || \sim T_{frame}), \mathsf{16B}, \mathsf{"derive\_picture\_key"})$  

## Secrets in Decoder Flash
```
0x1004_0000┌───────────────────────────┐
           │Static Random Bytes        │
           │                           │
0x1004_2000├───────────────────────────┤
           │Frame Key (16B)            │
           │Subscription Key (16B)     │
           │                           │
0x1004_4000├───────────────────────────┤
           │Channel 0 Subscription     │
           │Magic (4B), Chan. ID (4B)  │
           │Magic (4B), Chan. ID (4B)  │
           │~Magic (4B), ~Chan. ID (4B)│
           │~Magic (4B), ~Chan. ID (4B)│
           │Start Timestamp (8B)       │
           │End Timestamp (8B)         │
           │~Start Timestamp (8B)      │
           │~End Timestamp (8B)        │
           │Channel Secret 1/2 (16B)   │
           │~Channel Secret 1/2 (16B)  │
           │Channel Secret 2/2 (16B)   │
           │~Channel Secret 2/2 (16B)  │
           │                           │
0x1004_6000├───────────────────────────┤
           │Channel 1 Subscription     │
           │                           │
0x1004_8000├───────────────────────────┤
           │Channel 2 Subscription     │
           │                           │
0x1004_A000├───────────────────────────┤
           │Channel 3 Subscription     │
           │                           │
0x1004_C000├───────────────────────────┤
           │Channel 4 Subscription     │
           │                           │
0x1004_E000├───────────────────────────┤
           │Channel 5 Subscription     │
           │                           │
0x1005_0000├───────────────────────────┤
           │Channel 6 Subscription     │
           │                           │
0x1005_2000├───────────────────────────┤
           │Channel 7 Subscription     │
           │                           │
0x1005_4000├───────────────────────────┤
           │Channel 8 Subscription     │
           │                           │
0x1005_6000└───────────────────────────┘
```