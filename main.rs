use std::io;

fn main() {

    println!("Enter your message");
    let mut message = String::new();
    io::stdin().read_line(&mut message);
    println!("Choose the rotors");

    let alphabet: [char; 26] = [
        'a', 'b', 'c', 'd', 'e',
        'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y',
        'z'
    ];

    let _ciphertex: String = encryp(&*message, alphabet);
    println!("This is the ciphertext: {} ", _ciphertex);
    let _message_decryp: String = decryp(_ciphertex, alphabet);
    println!("This is the message: {} ", _message_decryp);
}

fn encryp(_message: &str, _alphabet: [char; 26]) -> String {
    let message_char: Vec<char> = _message.chars().collect();
    let mut ciphertext: String = String::new();
    
    let mut i = 0;
    while i != message_char.len()-1 {
        let mut count = 0;
        let mut j = 0;
        while j != _alphabet.len() {
            if message_char[i] != _alphabet[count] {
                count = count + 1;
            }
            j = j + 1;
        }

        if message_char[i] == " ".parse().unwrap() {
            ciphertext.push("@".parse().unwrap())
        }else{
            count = rotor(count, message_char[i]);
            ciphertext.push(_alphabet[count].to_string().parse().unwrap());
        }
        
        i = i + 1;
    }
    return ciphertext;
}

fn decryp(_ciphertext: String, _alphabet: [char; 26]) -> String {
    let ciphertext_char: Vec<char> = _ciphertext.chars().collect();
    let mut message: String = String::new();

    let mut i = 0;
    while i != ciphertext_char.len() {
        let mut count = 0;
        let mut j = 0;
        while j != _alphabet.len() {
            if ciphertext_char[i] != _alphabet[count] {
                count = count + 1;
            }
            j = j + 1;
        }

        if ciphertext_char[i] == "@".parse().unwrap() {
            message.push(" ".parse().unwrap())
        }else{
            count = rotor(count, ciphertext_char[i]);
            message.push(_alphabet[count].to_string().parse().unwrap());
        }

        i = i + 1;
    }
    return message;
}

fn rotor(count: usize, mess_or_cipher: char) -> usize {

    println!("What rotor do you want?\n1. rotor 1\n2. rotor 2");
    let opcion_1: u32;
    let mut opcion = String::new();
    io::stdin().read_line(&mut opcion);
    opcion_1 = opcion.trim().parse().unwrap();

    if opcion_1 == 1 && mess_or_cipher == 'Z' {
        return 0;
    } else if opcion_1 == 1 && mess_or_cipher != 'Z' {
        return count + 1;
    } else if opcion_1 == 2 && mess_or_cipher == 'A' {
        return 25;
    } else {
        return count - 1;
    }
}
