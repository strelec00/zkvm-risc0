use mnist_methods::{MNIST_ELF};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use risc0_zkvm::serde::from_slice;

pub fn run_inference(input: &[i32; 784]) -> (Receipt, i32) {
    let env = ExecutorEnv::builder()
        .write(&input.to_vec()) // pretvara slice u vektor za upis
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    let session = prover.prove(env, MNIST_ELF).unwrap();
    let receipt = session.receipt;

    let max_idx: i32 = from_slice(&*receipt.journal.bytes).unwrap();
    (receipt, max_idx)
}
