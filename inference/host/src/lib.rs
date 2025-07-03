use mnist_methods::MNIST_ELF as METHOD_ELF;
use risc0_zkvm::{
    default_prover, ExecutorEnv, InnerReceipt, ProverOpts, VerifierContext,
};
use risc0_zkvm::serde::from_slice;

use serde_json::json;
use std::fs::File;
use std::io::Write;

pub fn export_receipt(input: &[i32; 784]) {
    // 1️⃣ Build executor environment with input
    let env = ExecutorEnv::builder()
        .write(&input.to_vec()).unwrap()
        .build().unwrap();

    // 2️⃣ Run prover with Groth16 config and explicit context
    let prover = default_prover();

    let prove_info = prover
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            METHOD_ELF,
            &ProverOpts::groth16(),
        )
        .expect("Proving failed");

    let receipt = prove_info.receipt;

    // 3️⃣ Decode predicted digit from the journal
    let predicted: i32 = from_slice(&receipt.journal.bytes).unwrap();
    println!("Predviđeno: {}", predicted);

    // 4️⃣ Extract Groth16 seal
    let groth16_seal = match receipt.inner {
        InnerReceipt::Groth16(ref g) => g.seal.clone(),
        _ => panic!("Očekivao Groth16 receipt, dobio nešto drugo."),
    };

    // 5️⃣ Write seal to seal.json
    let seal_json = json!({
        "seal": base64::encode(&groth16_seal)
    });

    let mut file = File::create("seal.json").expect("Ne mogu stvoriti seal.json");
    write!(file, "{}", seal_json.to_string()).expect("Ne mogu zapisati u seal.json");

    println!("✅ Groth16 seal spremljen u seal.json. Pokreni Docker prover za proof.json.");
}
