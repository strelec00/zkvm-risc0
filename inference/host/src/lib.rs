use mnist_methods::MNIST_ELF;
use risc0_zkvm::{ExecutorEnv, ProverOpts, Receipt, InnerReceipt};
use risc0_zkvm::serde::from_slice;

use serde_json::json;
use std::fs::File;
use std::io::Write;

pub fn export_receipt(input: &[i32; 784]) {
    // 1️⃣ Build executor environment with input
    let env = ExecutorEnv::builder()
        .write(&input.to_vec()).unwrap()
        .build().unwrap();

    // 2️⃣ Configure prover options for Groth16 output
    let prover_opts = ProverOpts::groth16();

    // 3️⃣ Run prover with Groth16 option
    let prover = risc0_zkvm::default_prover();
    let session = prover.prove_with_opts(env, MNIST_ELF, &prover_opts).unwrap();
    let receipt: Receipt = session.receipt;

    // 4️⃣ Decode predicted digit from the journal
    let predicted: i32 = from_slice(&receipt.journal.bytes).unwrap();
    println!("Predviđeno: {}", predicted);

    // 5️⃣ Extract the Groth16 seal
    let groth16_seal = match receipt.inner {
        InnerReceipt::Groth16(ref g) => g.seal.clone(),
        _ => panic!("Očekivao Groth16 receipt, dobio nešto drugo."),
    };

    // 6️⃣ Write the seal to seal.json for use with the Groth16 prover Docker
    let seal_json = json!({
        "seal": base64::encode(&groth16_seal)
    });

    let mut file = File::create("seal.json").expect("Ne mogu stvoriti seal.json");
    write!(file, "{}", seal_json.to_string()).expect("Ne mogu zapisati u seal.json");

    println!("✅ Groth16 seal spremljen u seal.json. Pokreni Docker prover za proof.json.");
}
