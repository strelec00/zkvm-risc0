use multiply_methods::MULTIPLY_ELF; // It is a binary file of multiply_method
use risc0_zkvm::{
    default_prover,
        ExecutorEnv,
};

fn main() {

    // Declaring our secret input params
    let a: u64 = 17;
    let b: u64 = 23;

    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .write(&a)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, MULTIPLY_ELF).unwrap().receipt;

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );
    // Print an assertion
    println!("Hello, world! I know the factors of {}, and I can prove it!", c);

    // Let's serialize the receipt so we can save it to an file for verifier program to verify.

    (receipt, c);

}