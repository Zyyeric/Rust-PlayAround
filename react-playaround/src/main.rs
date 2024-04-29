use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::Path;

fn main() -> io::Result<()> {
    // Step 1: Prompt user for the filename
    println!("Please enter the filename of the text file:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let filename = input.trim();

    // Step 2: Check if the file exists
    let path = Path::new(filename);
    if !path.exists() {
        // Step 3: File does not exist
        println!("Error: File does not exist.");
        return Ok(());
    }

    // Step 4: Read the file and convert to uppercase
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let uppercased_contents = contents.to_uppercase();

    // Step 5: Write the uppercase content to a new file
    let output_file = File::create("output.txt")?;
    let mut writer = BufWriter::new(output_file);
    writer.write_all(uppercased_contents.as_bytes())?;

    println!("The file was processed and output.txt was created.");

    Ok(())
}
