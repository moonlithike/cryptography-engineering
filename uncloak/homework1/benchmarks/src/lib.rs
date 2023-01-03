use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use rand::rngs::ThreadRng;

fn main() {
   let mut rng = rand::thread_rng();

    let rsa: RSA = RSA::new();
    let enc_data = rsa.encrypt(b"string");
    let dec_data = rsa.decrypt(&enc_data);
}

pub struct RSA {
    pub private: RsaPrivateKey,
    pub public: RsaPublicKey,
}

impl RSA {
    pub fn new() -> Self {
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rand::thread_rng(), bits).unwrap();
        let pub_key = RsaPublicKey::from(&priv_key);

        RSA { private: priv_key, public: pub_key }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        self.public.encrypt(
            &mut rand::thread_rng(), 
            PaddingScheme::new_pkcs1v15_encrypt(), &data[..]
        ).unwrap()
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        self.private.decrypt(
            PaddingScheme::new_pkcs1v15_encrypt(), 
            &data
        ).unwrap()
    } 

}

