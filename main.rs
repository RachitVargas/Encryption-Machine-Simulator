use std::io;

fn main() {
    let message = "HELLOWORLD";

    let alphabet: [char; 26] = [
        'A', 'B', 'C', 'D', 'E',
        'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O',
        'P', 'Q', 'R', 'S', 'T',
        'U', 'V', 'W', 'X', 'Y',
        'Z'
    ];

    let _ciphertex: String = encryp(message, alphabet);
    println!("This is the ciphertext: {} ", _ciphertex);
    let _message_decryp: String = decryp(_ciphertex, alphabet);
    println!("This is the message: {} ", _message_decryp);
}

fn encryp(_message: &str, _alphabet: [char; 26]) -> String {
    let message_char: Vec<char> = _message.chars().collect();
    let mut ciphertext: String = String::new();

    let mut i = 0;
    while i != message_char.len() {
        let mut count = 0;
        let mut j = 0;
        while j != _alphabet.len() {
            if message_char[i] != _alphabet[count] {
                count = count + 1;
            }
            j = j + 1;
        }
        if count < 25 {
            count = count + 1;
            ciphertext.push(_alphabet[count].to_string().parse().unwrap());
        }else{
            ciphertext.push(_alphabet[0].to_string().parse().unwrap());
        }
        i = i+1;
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
        if count == 0 {
            message.push(_alphabet[25].to_string().parse().unwrap());
        }else{
            count = count-1;
            message.push(_alphabet[count].to_string().parse().unwrap());
        }
        i = i + 1;
    }
    return message;
}

fn rotor (count:usize)-> usize{

    println!("What rotor do you want?\n1. rotor 1\n2. rotor 2");
    let opcion_1:u32;
    let mut opcion= String::new();
    io::stdin().read_line(&mut opcion);
    opcion_1 = opcion.trim().parse().unwrap();

    if opcion_1 == 1{
        return count+1
    }
    return count-1;
}
