pub mod ciphers;

fn main() {
    println!("Hello, world!\n");
    let plaintext = "Hello.";
    let shift = 3;
    let key = "key";
    let caesar = ciphers::caesar(plaintext, shift);
    let vigenere = ciphers::vigenere(plaintext, key);
    let morse = ciphers::morse_encode(plaintext);

    println!("Plaintext: {}\n", plaintext);
    println!("CAESAR\nShift: {}\nCiphertext: {}\n", shift, caesar);
    println!("VIGENERE\nKey: {}\nCiphertext: {}\n", key, vigenere);
    println!("MORSE\nCiphertext: {}\n", morse);
}