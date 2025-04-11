fn main() {
    // Input the file name from the command line arguments
    let mut input = String::new();

    println!("Enter the file name (without extension): ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let filename = input.trim(); // Remove any trailing newline characters

    let dir = "./programs/";

    // Read the file contents into a string
    let file_path = format!("{}{}.c", dir, filename);
    
    let contents = std::fs::read_to_string(&file_path).unwrap_or_else(|_| {
        eprintln!("Error: Could not read file {}", file_path);
        std::process::exit(1);
    });

    // Print the file contents to the console
    println!("{}", contents);
}