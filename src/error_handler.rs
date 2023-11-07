use std::io::{self, Read};

pub fn get_message() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)
        .expect("Failed to read from stdin");
    buffer
}

fn xor_aes_error(algo: &str, message: &str, block: bool, key: &str, content: &str) -> bool {
    let message_list = vec!["-c", "-d"];

    if algo == "-xor" && (!block || !message_list.contains(&message) || algo == "-xor" && content.len() != key.len()) {
        return true;
    }
    false
}

pub fn error_handler<'a>(args: &'a [String], content: &str) -> Result<&'a [String], &'static str> {
    if let [_, algo, message, block, key] = args {
        if xor_aes_error(algo, &message, block == "-b", key, content) {
            return Err("Error in XOR or AES validation");
        }
    } else {
        println!("Not sufficient args");
    }
    Ok(args)
}
