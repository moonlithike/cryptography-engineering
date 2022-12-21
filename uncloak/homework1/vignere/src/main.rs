// $ cargo run
// enter the text you would like to encrypt
// tothemoonandbeyond
// enter the key
// moonlight
// your ciphertext is: fchupuuvgmbropguuw
// now enter a key to decrypt the ciphertext:
// moonlight
// your decrypted text is: tothemoonandbeyond


use std::io;

fn main() {
    println!("enter the text you would like to encrypt");
    let mut plaintext = String::new();
    io::stdin().read_line(&mut plaintext).unwrap();
    plaintext.pop();

    println!("enter the key");
    let mut enc_key = String::new();
    io::stdin().read_line(&mut enc_key).unwrap();
    enc_key.pop();


    let ciphertext = encrypt(plaintext, enc_key);

    println!("your ciphertext is: {}", ciphertext);

    println!("now enter a key to decrypt the ciphertext:");
    let mut dec_key = String::new();
    io::stdin().read_line(&mut dec_key).unwrap();
    dec_key.pop();

    println!("your decrypted text is: {}", decrypt(ciphertext, dec_key));


}

fn encrypt(plaintext: String, key: String) -> String {
    let mut ciphertext = String::new();
    let key_chars: Vec<_> = key.chars().collect();

    for (i, c) in plaintext.chars().enumerate() {
        // cast characters into ascii integer values and back
        // use remainder op (%) to select index from key vector and wrap around alphabet
        ciphertext.push(
            ((c as u8 - 97 + key_chars[i % key_chars.len()] as u8 - 97) % 26 + 97 ) as char
        );
    }
    ciphertext
}

fn decrypt(ciphertext: String, key: String) -> String {
    let mut plaintext = String::new();
    let key_chars: Vec<_> = key.chars().collect();

    for (i, c) in ciphertext.chars().enumerate() {
        // same as above, but use remainder op to perform negative module to reverse op:
        //     ((a % b) + b) % b)
       plaintext.push(
            (((((c as u8 as i8 - key_chars[i % key_chars.len()] as u8 as i8) % 26) + 26 ) % 26) + 97 ) as u8 as char
        );
    }
    plaintext
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert_eq!(
            encrypt(String::from("aaaaa"), String::from("bbbbb")),
            String::from("bbbbb")
        );
        assert_eq!(
            encrypt(String::from("zzzzz"), String::from("zzzzz")),
            String::from("yyyyy")
        );
        assert_eq!(
            encrypt(String::from("asjkasjsakkjflaoepqcnvcmzbv"), String::from("keyy")),
            String::from("kwhikwhqkoihppymotoaxzakjft")
        );
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(
            decrypt(String::from("bbbbb"), String::from("bbbbb")),
            String::from("aaaaa")
        );
        assert_eq!(
            decrypt(String::from("yyyyy"), String::from("zzzzz")),
            String::from("zzzzz")
        );
        assert_eq!(
            decrypt(String::from("kwhikwhqkoihppymotoaxzakjft"), String::from("keyy")),
            String::from("asjkasjsakkjflaoepqcnvcmzbv")
        );
    }
}