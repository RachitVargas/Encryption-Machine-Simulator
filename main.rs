
fn main() {

    //MESSAGE
    let message = "HOLAMUNDOY";

    // ALPHABET
    let alphabet:[char; 26] = [
        'A', 'B', 'C', 'D', 'E',
        'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O',
        'P', 'Q', 'R', 'S', 'T',
        'U', 'V', 'W', 'X', 'Y',
        'Z'
    ];

    let _ciphertex:String = encryp(message, alphabet);
    println!("This is the ciphertext: {} ",_ciphertex);
    let _message_decryp:String = decryp(_ciphertex, alphabet);
    println!("This is the message: {} ", _message_decryp);

}

fn encryp(_message:&str, _alphabet:[char;26]) -> String{

    let message_char: Vec<char> = _message.chars().collect();
    let mut ciphertext:String = String::new();

    let mut i = 0;
    while i != message_char.len() {
        let mut count = 0;
        let mut j = 0;
        while j != _alphabet.len() {
            if message_char[i] != _alphabet[count] {
                count = count + 1;
            }
            j = j+1;
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

fn decryp (_ciphertext:String,_alphabet:[char; 26]) -> String {
    let ciphertext_char: Vec<char> = _ciphertext.chars().collect();
    let mut message:String = String::new();

    let mut i = 0;
    while i != ciphertext_char.len() {
        let mut count = 0;
        let mut j = 0;
        while j != _alphabet.len() {
            if ciphertext_char[i] != _alphabet[count] {
                count = count + 1;
            }
            j = j+1;
        }
        if count < 25 || count > 0 {
            count = count - 1;
            message.push(_alphabet[count].to_string().parse().unwrap());
        }else{
            message.push(_alphabet[25].to_string().parse().unwrap());
        }
        i = i+1;
    }

    return message;
}
