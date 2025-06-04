use mnist_methods::MNIST_ELF;
use risc0_zkvm::{seal_to_json, ExecutorEnv, ProverOpts, Receipt};
use std::fs::File;
use std::io::Cursor;

pub fn export_receipt(input: &[i32; 784]) {
    // 1. Priprema okruženja
    let env = ExecutorEnv::builder()
        .write(&input.to_vec()).unwrap()
        .build().unwrap();

    // 2. Create prover
    let prover = risc0_zkvm::default_prover();

    // 3. First prove with default settings to get composite receipt
    let session = prover.prove(env, MNIST_ELF).unwrap();
    let receipt: Receipt = session.receipt;

    // 4. Ispis predikcije iz journal
    let predicted: i32 = receipt.journal.decode().unwrap();
    println!("Predviđeno: {}", predicted);

    // 5. Compress the receipt to succinct form
    let succinct_receipt = prover.compress(&ProverOpts::succinct(), &receipt).unwrap();

    // 6. Extract the actual SuccinctReceipt from the compressed Receipt
    let succinct_inner = match succinct_receipt.inner {
        risc0_zkvm::InnerReceipt::Succinct(succinct_receipt) => succinct_receipt,
        _ => panic!("Expected succinct receipt after compression"),
    };

    // 7. Serijalizacija SuccinctReceipt objekta
    let encoded = bincode::serialize(&succinct_inner).unwrap();
    let reader = Cursor::new(encoded);

    // 8. JSON zapis
    let mut file = File::create("input.json").unwrap();
    seal_to_json(reader, &mut file).unwrap();

    println!("Successfully exported succinct receipt to input.json");
}