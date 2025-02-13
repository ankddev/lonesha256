use lonesha256::lonesha256;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return Ok(());
    }

    let mut file = File::open(&args[1])?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hash = [0u8; 32];
    lonesha256(&mut hash, &buffer)?;

    println!("SHA256 hash of {}: {}", args[1], 
        hash.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>());
    Ok(())
}
