use std::io;

fn main() {

    menu();
    let select_option:u32 = selectOp();
    let mut message:String = String::new();
    let mut key:String = String::new();
    let mut ciphertext:String = String::new();
    let mut message_decryp: String = String::new();

    if(select_option == 1){

        message = enter_message();
        println!("This is your message: {}", message);
        ciphertext = encryp(message);

    }else if(select_option == 2){

        println!("This is the ciphertex: {} ", ciphertext);

    }else if(select_option == 3){

        message_decryp = decryp();

    }else{
        println!("Sorry, we have a problem with the option selected");
    }
}

fn decryp() -> String{
    let me: &str = "Hi";
    return (me).parse().unwrap();

}

fn encryp(message:String) -> String{
    let me: &str = "Hi";
    return (me).parse().unwrap();
}

fn enter_message() -> String{
    println!("Enter the message");
    let mut message:String = String::new();
    io::stdin().read_line(&mut message);
    return (message);
}

fn selectOp() -> u32 {
    let mut option = String::new();
    io::stdin().read_line(&mut option);
    let option:u32 = option.trim().parse().unwrap();
    return (option);
}

fn menu(){
    println!("Hi, welcome to enigma code!");
    println!("Please choose a option");
    println!("1. Enconding a message\n2. Get the ciphertex\n3. Get the message");
}
