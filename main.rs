mod crypto;
use crypto::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: {} <encrypt|decrypt> <input_file> <output_file>", args[0]);
        return;
    }

    let mode = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    let data = fs::read(input_file).expect("Failed to read input file");

    // Generate RSA keys for demo (can store/load for real usage)
    let (private_key, public_key) = generate_rsa_keys();

    match mode.as_str() {
        "encrypt" => {
            let aes_key: [u8;32] = rand::random();
            let (ciphertext, iv) = encrypt_aes(&aes_key, &data);
            let encrypted_key = encrypt_rsa(&public_key, &aes_key);
            
            fs::write(output_file, ciphertext).expect("Failed to write encrypted file");
            fs::write("aes_key.enc", encrypted_key).expect("Failed to write AES key");
            fs::write("iv.bin", iv).expect("Failed to write IV");
            println!("File encrypted successfully!");
        },
        "decrypt" => {
            let encrypted_key = fs::read("aes_key.enc").expect("Failed to read AES key");
            let iv = fs::read("iv.bin").expect("Failed to read IV");
            let aes_key = decrypt_rsa(&private_key, &encrypted_key);

            let decrypted = decrypt_aes(&aes_key, &data, &iv);
            fs::write(output_file, decrypted).expect("Failed to write decrypted file");
            println!("File decrypted successfully!");
        },
        _ => println!("Invalid mode. Use encrypt or decrypt."),
    }
}
