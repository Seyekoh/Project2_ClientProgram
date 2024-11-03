use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::fs::File;
use std::path::Path;
use client_program::encode_to_base64;
use std::env;

fn main() {
    if let Err(error) = start_data_transfer() {
        eprintln!("Error during data transfer: {}", error);
    }
}

fn start_data_transfer() -> io::Result<()> {
    // Print the current working directory
    println!("Current directory: {:?}", env::current_dir()?);

    // Define input file path
    let file_path = Path::new("data/ALBNM/branch_weekly_sales.txt");

    // Check if the file exists before trying to open it
    if !file_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("File not found: {:?}", file_path)));
    }

    // Read file content
    let mut file = File::open(&file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Encode the content to Base64
    let encoded_content = encode_to_base64(&content);

    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080")?; 
    println!("Connected to the server");

    // Send branch code
    let branch_code = "bcode~ALBNM";
    stream.write_all(branch_code.as_bytes())?;
    println!("Sent branch code: {}", branch_code);

    // Wait for OK response
    let mut response = [0; 2];
    let bytes_read = stream.read(&mut response)?;
    if bytes_read != 2 {
        panic!("Expected 2 bytes, but received: {}", bytes_read);
    }
    if &response != b"OK" {
        panic!("Unexpected response from the server");
    }
    println!("Received OK from the server");

    // Send the Base64 encoded content enclosed in '~'
    let data_message = format!("~{}~", encoded_content);
    stream.write_all(data_message.as_bytes())?;
    println!("Sent encoded data");

    // Wait for the final OK response
    stream.read_exact(&mut response)?;
    if &response != b"OK" {
        panic!("File transfer failed");
    }
    println!("File transfered successfully");

    // Close the connection
    Ok(())
}
