use mnist_methods::MNIST_ELF;
use risc0_zkvm::{ExecutorEnv, ProverOpts, Receipt};
use risc0_zkvm::serde::from_slice;
use risc0_groth16::to_json;
use std::fs::File;
use std::io::{Cursor, Write};

pub fn export_receipt(input: &[i32; 784]) {
    // Izgradnja izvršnog okruženja
    let env = ExecutorEnv::builder()
        .write(&input.to_vec()).unwrap()
        .build().unwrap();

    // Konfiguriranje opcija prover-a za Groth16
    let prover_opts = ProverOpts::default();
    // Provođenje dokazivanja
    let prover = risc0_zkvm::default_prover();
    let session = prover.prove_with_opts(env, MNIST_ELF, &prover_opts).unwrap();
    let receipt: Receipt = session.receipt;


    // Deserijalizacija predviđene vrijednosti iz dnevnika
    let predicted: i32 = from_slice(&*receipt.journal.bytes).unwrap();
    println!("Predviđeno: {}", predicted);


}