# rust-file-encryption2.0

# Rust File Encryption Tool (AES-256-CBC + RSA)

Secure file encryption tool written in Rust supporting files of any size. Uses **AES-256 in CBC mode** for encryption and **RSA** to protect the AES key.

## Features
- Encrypt/decrypt files securely
- AES key encryption with RSA
- CBC mode with PKCS7 padding (any file size)
- CLI interface: `encrypt <input> <output>` / `decrypt <input> <output>`

## Usage
1. Clone the repo
2. Build: `cargo build --release`
3. Encrypt a file:
```bash
./target/release/rust-file-encryption encrypt example.txt encrypted.dat
