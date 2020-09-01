use sodiumoxide::crypto::box_;
use std::str;

type SecretKey = sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::SecretKey;
type PublicKey = sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey;
type Nonce = sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::Nonce;

pub struct Keys {
    public: PublicKey,
    secret: SecretKey,
}

pub struct KeysRef<'a> {
    public: &'a PublicKey,
    secret: &'a SecretKey,
}

pub fn generate_keys() -> Keys {
    let (pub_key, sec_key) = box_::gen_keypair();
    Keys {
        public: pub_key,
        secret: sec_key,
    }
}

pub fn encrypt_message(message: &str, keys: KeysRef, nonce: &Nonce) -> Vec<u8> {
    let byte_string_message = message.as_bytes();
    let ciphertext = box_::seal(byte_string_message, nonce, keys.public, keys.secret);
    ciphertext
}

pub fn decrypt_message(message: &Vec<u8>, keys: KeysRef, nonce: &Nonce) -> String {
    let byte_message = box_::open(message, &nonce, keys.public, keys.secret).unwrap();
    let message = str::from_utf8(&byte_message).unwrap();
    message.to_string()
}

pub fn hello() {
    println!("Hello!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
