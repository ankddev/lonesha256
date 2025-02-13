mod sys;

use std::convert::TryInto;
use std::fmt::{self, Display, Formatter};
use std::error::Error;

/// Size of SHA256 hash output in bytes
pub const HASH_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sha256Error {
    ComputationError,
    BufferSizeError,
}

impl Display for Sha256Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Sha256Error::ComputationError => write!(f, "SHA256 computation failed"),
            Sha256Error::BufferSizeError => write!(f, "Output buffer must be 32 bytes long"),
        }
    }
}

impl Error for Sha256Error {}

/// Computes SHA256 hash of input data
///
/// # Example
/// ```
/// use lonesha256::lonesha256;
///
/// let data = b"hello world";
/// let mut output = [0u8; 32];
/// lonesha256(&mut output, data).unwrap();
/// ```
pub fn lonesha256(out: &mut [u8], input: &[u8]) -> Result<(), Sha256Error> {
    if out.len() != HASH_SIZE {
        return Err(Sha256Error::BufferSizeError);
    }

    let result = unsafe {
        sys::lonesha256(
            out.as_mut_ptr(),
            input.as_ptr(),
            input.len().try_into().map_err(|_| Sha256Error::ComputationError)?
        )
    };

    match result {
        0 => Ok(()),
        _ => Err(Sha256Error::ComputationError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_hash() {
        let input = b"hello world";
        let mut output = [0u8; HASH_SIZE];
        lonesha256(&mut output, input).unwrap();
        assert_eq!(
            output.iter().map(|b| format!("{:02x}", b)).collect::<String>(),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_buffer_size_error() {
        let mut small_buf = [0u8; 16];
        assert!(matches!(
            lonesha256(&mut small_buf, b"test"),
            Err(Sha256Error::BufferSizeError)
        ));
    }

    #[test]
    fn test_hash_stability() {
        let input = b"test stability";
        let mut hash1 = [0u8; HASH_SIZE];
        let mut hash2 = [0u8; HASH_SIZE];
        
        lonesha256(&mut hash1, input).unwrap();
        lonesha256(&mut hash2, input).unwrap();
        
        assert_eq!(hash1, hash2, "Same input should produce same hash");
    }

    #[test]
    fn test_different_inputs() {
        let mut hash1 = [0u8; HASH_SIZE];
        let mut hash2 = [0u8; HASH_SIZE];
        
        lonesha256(&mut hash1, b"input1").unwrap();
        lonesha256(&mut hash2, b"input2").unwrap();
        
        assert_ne!(hash1, hash2, "Different inputs should produce different hashes");
    }

    #[test]
    fn test_empty_vec() {
        let input: Vec<u8> = Vec::new();
        let mut hash = [0u8; HASH_SIZE];
        lonesha256(&mut hash, &input).unwrap();
        
        // Known hash for empty input
        let expected = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14,
            0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24,
            0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c,
            0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
        ];
        assert_eq!(hash, expected);
    }
}
