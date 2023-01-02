use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};

fn main() {
    // let mut rng = rand::thread_rng();
    // let bits = 2048;
    // let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    // let pub_key = RsaPublicKey::from(&priv_key);
    let keys = gen_keys();

// Encrypt
    let data = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

}

fn gen_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut ring = rand::thread_rng();
    let bits = 2048;
    (
        RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key"),
        RsaPublicKey::from(&priv_key)
    )
}

fn encrypt()