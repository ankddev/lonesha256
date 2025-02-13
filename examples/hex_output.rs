use lonesha256::lonesha256;

fn to_hex_string(hash: &[u8]) -> String {
    hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn to_hex_uppercase(hash: &[u8]) -> String {
    hash.iter()
        .map(|b| format!("{:02X}", b))
        .collect()
}

fn to_hex_grouped(hash: &[u8]) -> String {
    hash.chunks(4)
        .map(|chunk| chunk.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let input = b"Test different hex output formats";
    let mut hash = [0u8; 32];
    lonesha256(&mut hash, input).expect("Failed to compute hash");

    println!("Input: {}", String::from_utf8_lossy(input));
    println!("\nDifferent output formats:");
    println!("Standard:  {}", to_hex_string(&hash));
    println!("Uppercase: {}", to_hex_uppercase(&hash));
    println!("Grouped:   {}", to_hex_grouped(&hash));
}
