use secp256k1::{Secp256k1, SecretKey, PublicKey, rand::rngs::OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit};
use aes_gcm::aead::Aead;
use rand::Rng;
use hex;

#[derive(Clone)]
pub struct Crypto {
    secp: Secp256k1<secp256k1::All>,
}

impl Crypto {
    pub fn new() -> Self {
        Crypto { secp: Secp256k1::new() }
    }

    pub fn generate_keypair(&self) -> (SecretKey, PublicKey) {
        let mut rng = OsRng;
        let secret_key = SecretKey::new(&mut rng);
        let public_key = PublicKey::from_secret_key(&self.secp, &secret_key);
        (secret_key, public_key)
    }

    pub fn encrypt_message(&self, message: &str, shared_secret: &[u8; 32]) -> (String, String) {
        let key = Key::<Aes256Gcm>::from_slice(shared_secret);
        let cipher = Aes256Gcm::new(key);
        let nonce_bytes = rand::thread_rng().gen::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, message.as_bytes()).unwrap();
        (
            hex::encode(ciphertext),
            hex::encode(nonce_bytes),
        )
    }

    pub fn decrypt_message(&self, ciphertext: &str, nonce: &str, shared_secret: &[u8; 32]) -> String {
        let key = Key::<Aes256Gcm>::from_slice(shared_secret);
        let cipher = Aes256Gcm::new(key);
        let nonce_bytes = hex::decode(nonce).unwrap();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let plaintext = cipher.decrypt(nonce, hex::decode(ciphertext).unwrap().as_ref()).unwrap();
        String::from_utf8(plaintext).unwrap()
    }

    #[allow(dead_code)]
    pub fn derive_shared_secret(&self, _secret_key: &SecretKey, _public_key: &PublicKey) -> [u8; 32] {
        // Simplified for demo; use proper ECDH in production
        let shared = [0u8; 32];
        shared
    }
}