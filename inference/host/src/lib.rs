use mnist_methods::MNIST_ELF;
use risc0_zkvm::{ExecutorEnv, ProverOpts, Receipt, InnerReceipt};
use risc0_zkvm::serde::from_slice;

use std::io::Write;
use base64;

pub fn export_receipt(input: &[i32; 784]) {
    // 1️⃣ Build executor environment
    let env = ExecutorEnv::builder()
        .write(&input.to_vec()).unwrap()
        .build().unwrap();

    // 2️⃣ Koristi Groth16 prover opcije
    let prover_opts = ProverOpts::default();

    // 3️⃣ Pokreni prover
    let prover = risc0_zkvm::default_prover();
    let session = prover.prove_with_opts(env, MNIST_ELF, &prover_opts).unwrap();
    let receipt: Receipt = session.receipt;

    // 4️⃣ Parsiraj predikciju
    let predicted: i32 = from_slice(&receipt.journal.bytes).unwrap();
    println!("Predviđeno: {}", predicted);

}