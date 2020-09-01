# CRYPTO TEST

The example folder contains simple cryptographic scenarios by using the local `sodiumoxide_adapter` crate 

# What is sodiumoxide ?

**Sodiumoxide is the rust binding for libsodium, which in turn is the maintained fork (portable, cross-compilable, installable, packageable) of NaCl.**

*NaCl (stable release in 2009) was developped to eliminate known cryptographic disasters :*

- Protection against cache-timing attacks : 

>"The secret AES key inside the kernel
influences the table-load addresses, which in turn influence the state of the CPU
cache, which in turn influences measurable timings of the attack process; the
attack process computes the AES key from this leaked information."

- Protection against remote timing attacks :

>"The underlying problem is that most scalar-multiplication (and exponentia-
tion) algorithms involve data flow from secret data into branch conditions: i.e.,
certain operations are carried out if and only if the key has certain properties."

- Protection against Bleichenbacher’s attack

>"In 1998 Bleichenbacher successfully decrypted an RSA-
encrypted SSL ciphertext by sending roughly one million variants of the cipher-
text to the server and observing the server’s responses."

- No "bad" randomness

>"In 2006 a Debian developer removed one critical
line of randomness-generation code from the OpenSSL package shipped with
Debian GNU/Linux.[...] Until this bug was discovered in 2008 (see [34]), OpenSSL keys
generated under Debian and Ubuntu were chosen from a set of size only 32768.
Breaking the encryption or authentication of any communication secured with
such a key was a matter of seconds."

>"Badly generated random numbers were
also involved in the recent collapse of the security system of Sony’s PlayStation
3 gaming console. Sony used the standard elliptic-curve digital-signature algo-
rithm, ECDSA, but ignored the ECDSA requirement of a new random secret
for each message: Sony simply used a constant value for all messages."

- No SPEED issue 

>"Cryptographic performance problems have frequently caused users to reduce
their cryptographic security levels or to turn off cryptography entirely."

- Protection against cryptographic primitives being broken 

>"In 2008 [...] by exploiting various weaknesses that had been
discovered in the MD5 hash function, they had created a rogue CA certificate.
They could, if they wanted, have impersonated any SSL site on the Internet."

# Why sodiumoxide / libsodium ?

- Easy to use 
- Fast
- Cross platform 

>"The design choices emphasize security and ease of use. But despite the emphasis on high security, primitives are faster across-the-board than most implementations."

- updated to the most secure parameters & keep backward compatibility

>"Sodium's high-level crypto_pwhash_* API currently leverages the Argon2id function on all platforms. This can change at any point in time, but it is guaranteed that a given version of libsodium can verify all hashes produced by all previous versions, from any platform. Applications don't have to worry about backward compatibility."

- Undefeated and widely validated up to 2020

# Refs

- "The security impact
of a new cryptographic library" Daniel J. Bernstein & Al : https://cr.yp.to/highspeed/coolnacl-20120725.pdf

- http://nacl.cr.yp.to/

- https://doc.libsodium.org/

- https://docs.rs/sodiumoxide/