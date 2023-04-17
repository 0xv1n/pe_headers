use pelite::pe64::{Pe, PeFile};
use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <PE file>", args[0]);
        process::exit(1);
    }

    let mut file = File::open(&args[1]).expect("Failed to open the file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read the file");

    let pe_file = match PeFile::from_bytes(&buffer) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to parse the PE file");
            process::exit(1);
        }
    };

    let dos_header = pe_file.dos_header();
    println!("DOS Header:");
    println!("{:#?}", dos_header);

    let file_header = pe_file.file_header();
    println!("\nFile Header:");
    println!("{:#?}", file_header);

    let optional_header = pe_file.optional_header();
    println!("\nOptional Header:");
    println!("{:#?}", optional_header);
}
