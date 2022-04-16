fn main() {
    let byte_escape = "I'm writing Ru\x73t!";
    println!("What are you doing? (\\x3F means \x73?) {}", byte_escape);
}