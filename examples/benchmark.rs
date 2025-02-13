use lonesha256::lonesha256;
use std::time::Instant;

fn benchmark_hash(size: usize, iterations: u32) {
    let data = vec![0x55; size];
    let mut hash = [0u8; 32];
    
    let start = Instant::now();
    for _ in 0..iterations {
        lonesha256(&mut hash, &data).expect("Hash computation failed");
    }
    let duration = start.elapsed();
    
    let mb_processed = (size * iterations as usize) as f64 / 1_000_000.0;
    let seconds = duration.as_secs_f64();
    let mb_per_sec = mb_processed / seconds;

    println!(
        "Size: {}KB, Iterations: {}, Time: {:.2}s, Speed: {:.2} MB/s",
        size / 1024,
        iterations,
        seconds,
        mb_per_sec
    );
}

fn main() {
    println!("Running SHA256 benchmarks...\n");
    
    // Test different sizes
    benchmark_hash(1024, 100_000);      // 1KB
    benchmark_hash(1024 * 1024, 100);   // 1MB
    benchmark_hash(10 * 1024 * 1024, 10); // 10MB
}
