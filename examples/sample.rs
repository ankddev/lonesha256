use lonesha256::lonesha256;

fn main() {
    // Example 1: Simple string hashing
    let input = b"Hello, world!";
    let mut hash = [0u8; 32];

    lonesha256(&mut hash, input).expect("Failed to compute hash");

    println!("Input: {}", String::from_utf8_lossy(input));
    println!(
        "Hash: {}",
        hash.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );

    // Example 2: Empty string hash
    let mut empty_hash = [0u8; 32];
    lonesha256(&mut empty_hash, b"").expect("Failed to compute empty hash");
    println!(
        "\nEmpty string hash: {}",
        empty_hash
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );

    // Example 3: Error handling
    let mut small_buf = [0u8; 16];
    match lonesha256(&mut small_buf, b"test") {
        Ok(_) => println!("Hash computed"),
        Err(e) => println!("Error: {}", e),
    }
}
