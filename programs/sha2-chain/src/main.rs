#![no_main]
use std::hint::black_box;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[cfg(feature = "sp1")] 
use sha2::{Digest, Sha256};
    
#[cfg(feature = "risc0")] 
use risc0_zkvm::sha::rust_crypto::{Sha256, Digest};


fn sha_chain(n: u32) -> [u8; 32] {
    let mut out = [0u8; 32];

    for _ in 0..n {
        let mut hasher = Sha256::new();
        hasher.update(out);
        let digest = &hasher.finalize();
        out = Into::<[u8; 32]>::into(*digest);
    }

    out
}

pub fn main() {
    let result = black_box(sha_chain(black_box(1_000_000)));
    println!("result: {:?}", result.len());
}
