// This code is borrowed from RISC Zero's benchmarks.
//
// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use regex::Regex;
use std::collections::HashMap;

fn main() {
    // let pattern = r"\b(and|all)\b".to_string();
    let pattern = r"\b(and|all|management|emulated|the|blockchain|is|project|)\b".to_string();

    #[cfg(feature = "sp1")]
    let text = sp1_zkvm::io::read::<String>();
    
    #[cfg(feature = "risc0")]
    let text = risc0_zkvm::guest::env::read::<String>();

    let re = Regex::new(&pattern).unwrap();
    let mut word_positions: HashMap<&str, Vec<usize>> = HashMap::new();
    for mat in re.find_iter(&text) {
        let word = mat.as_str();
        let position = mat.start();

        word_positions.entry(word).or_insert(Vec::new()).push(position);
    }

    #[cfg(feature = "sp1")]
    sp1_zkvm::io::commit(&word_positions);
    
    #[cfg(feature = "risc0")]
    risc0_zkvm::guest::env::commit(&word_positions); 
}
