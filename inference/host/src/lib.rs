use mnist_methods::MNIST_ELF;
use risc0_zkvm::{default_prover, seal_to_json, ExecutorEnv, Receipt};
use risc0_zkvm::serde::from_slice;
use std::fs::File;
use std::io::{Cursor, Write};


pub fn export_receipt(input: &[i32; 784]) {
    // Build execution environment
    let env = ExecutorEnv::builder()
        .write(&input.to_vec()).unwrap()
        .build().unwrap();

    // Prove the computation
    let prover = default_prover();
    let session = prover.prove(env, MNIST_ELF).unwrap();
    let receipt: Receipt = session.receipt;

    // Deserialize predicted value from receipt journal
    let predicted: i32 = from_slice(&*receipt.journal.bytes).unwrap();
    println!("Predicted: {}", predicted);

    // Serialize receipt to bytes
    let encoded_receipt = bincode::serialize(&receipt).unwrap();

    // Write the binary receipt file (optional)
    let mut bin_file = File::create("proof.bin").unwrap();
    bin_file.write_all(&encoded_receipt).unwrap();

    // Create a Cursor reader over the serialized receipt bytes
    let reader = Cursor::new(encoded_receipt);

    // Open a file for JSON output
    let mut json_file = File::create("proof.json").unwrap();

    // Convert the receipt binary to JSON and write to file
    seal_to_json(reader, &mut json_file).unwrap();

    // Now the proof.json contains the JSON version of the receipt
}
