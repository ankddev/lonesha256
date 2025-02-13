use lonesha256::lonesha256;

fn main() {
    // Hash multiple chunks of data
    let data_chunks: Vec<&[u8]> = vec![
        b"Part 1 of the message",
        b" - Part 2 of the message",
        b" - Final part of the message",
    ];

    // Combine chunks
    let full_data: Vec<u8> = data_chunks
        .iter()
        .flat_map(|chunk| chunk.iter().copied())
        .collect();

    // Hash combined data
    let mut hash = [0u8; 32];
    lonesha256(&mut hash, &full_data).expect("Failed to compute hash");

    println!("Full message: {}", String::from_utf8_lossy(&full_data));
    println!(
        "Hash: {}",
        hash.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );
}
