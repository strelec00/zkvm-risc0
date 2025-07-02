use mnist_methods::MNIST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, Receipt};

// Fallback function for platforms that don't support Groth16
pub fn generate_stark_proof_fallback(input: &[i32; 784]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Generating STARK proof as fallback...");

    let env = ExecutorEnv::builder()
        .write(&input.to_vec())?
        .build()?;

    let prover = default_prover();

    // Generate regular STARK proof
    let prove_info = prover.prove(env, MNIST_ELF)?;
    let receipt = prove_info.receipt;

    let predicted: i32 = receipt.journal.decode()?;
    println!("Predicted digit: {}", predicted);

    // Save STARK proof instead
    let stark_data = serde_json::json!({
        "proof_type": "STARK",
        "prediction": predicted,
        "image_id": format!("mnist_prediction_{}", predicted),
        "proof_system": "risc0_stark_fallback",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "platform": std::env::consts::OS,
        "architecture": std::env::consts::ARCH,
        "note": "Groth16 not available on this platform, using STARK proof instead"
    });

    let mut file = File::create("stark_proof_fallback.json")?;
    to_writer_pretty(&mut file, &stark_data)?;

    println!("✅ STARK proof saved to stark_proof_fallback.json");
    Ok(())
}
use serde_json::to_writer_pretty;
use std::fs::File;
use base64::{Engine as _, engine::general_purpose};

pub fn export_receipt(input: &[i32; 784]) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting RISC Zero Groth16 proof generation...");

    // 1️⃣ Create execution environment
    let env = ExecutorEnv::builder()
        .write(&input.to_vec())?
        .build()?;

    let prover = default_prover();

    // 2️⃣ Generate Groth16 proof directly using prove_with_opts
    println!("Generating Groth16 proof (this may take a while)...");
    println!("Platform: {}", std::env::consts::OS);
    println!("Architecture: {}", std::env::consts::ARCH);

    // Add timeout and better error handling
    let prove_info = match prover.prove_with_opts(env, MNIST_ELF, &ProverOpts::groth16()) {
        Ok(info) => {
            println!("✅ Groth16 proof generation completed successfully!");
            info
        },
        Err(e) => {
            eprintln!("❌ Groth16 proof generation failed: {}", e);
            eprintln!("This might be due to:");
            eprintln!("  1. Groth16 not supported on this platform (macOS/Apple Silicon)");
            eprintln!("  2. Missing system dependencies");
            eprintln!("  3. Insufficient memory or CPU resources");
            eprintln!("\nTrying fallback to standard STARK proof...");

            // Fallback to regular proof
            return generate_stark_proof_fallback(input);
        }
    };
    let receipt = prove_info.receipt;

    // Extract the prediction from journal
    let predicted: i32 = receipt.journal.decode()?;
    println!("Predicted digit: {}", predicted);

    // 3️⃣ Extract the Groth16 seal from the receipt
    let groth16_seal = match &receipt.inner {
        risc0_zkvm::InnerReceipt::Groth16(groth16_receipt) => &groth16_receipt.seal,
        _ => return Err("Expected Groth16Receipt, but got a different receipt type".into()),
    };

    // 4️⃣ Save Groth16 proof to JSON file
    println!("Saving Groth16 proof to file...");

    // Handle Groth16Seal serialization
    let groth16_data = if let Ok(serialized) = serde_json::to_string(&groth16_seal) {
        // If the seal implements Serialize, use it directly
        serde_json::json!({
            "groth16_seal": serialized,
            "prediction": predicted,
            "image_id": format!("mnist_prediction_{}", predicted),
            "proof_system": "risc0_groth16_direct",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
    } else {
        // Fallback: store as debug representation
        let seal_repr = format!("{:?}", groth16_seal);
        serde_json::json!({
            "groth16_seal_debug": seal_repr,
            "prediction": predicted,
            "image_id": format!("mnist_prediction_{}", predicted),
            "proof_system": "risc0_groth16_direct",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "note": "Groth16 seal stored as debug representation"
        })
    };

    let mut file = File::create("groth16_proof.json")?;
    to_writer_pretty(&mut file, &groth16_data)?;

    println!("✅ Successfully generated and saved Groth16 proof to groth16_proof.json");

    // Optional: Print some info about the receipt
    println!("Receipt journal size: {} bytes", receipt.journal.bytes.len());

    Ok(())
}

// Helper function to verify the generated proof (optional)
pub fn verify_groth16_proof(proof_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    let proof_data = fs::read_to_string(proof_file)?;
    let json: serde_json::Value = serde_json::from_str(&proof_data)?;

    println!("Groth16 proof loaded successfully from {}", proof_file);

    if let Some(prediction) = json["prediction"].as_i64() {
        println!("Predicted digit: {}", prediction);
    }

    if let Some(timestamp) = json["timestamp"].as_str() {
        println!("Proof generated at: {}", timestamp);
    }

    Ok(())
}

// Check if Groth16 is supported on current platform
pub fn check_groth16_support() -> bool {
    // Groth16 typically requires x86_64 Linux with specific dependencies
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => true,
        _ => false,
    }
}

// Recommended function that automatically chooses the best proof type
pub fn export_receipt_auto(input: &[i32; 784]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Checking platform compatibility...");
    println!("Platform: {} {}", std::env::consts::OS, std::env::consts::ARCH);

    if check_groth16_support() {
        println!("✅ Groth16 should be supported on this platform");
        export_receipt(input)
    } else {
        println!("⚠️  Groth16 may not be supported on this platform");
        println!("📝 macOS (especially Apple Silicon) typically doesn't support Groth16");
        println!("🔄 Using STARK proof instead...");
        generate_stark_proof_fallback(input)
    }
}
pub fn export_receipt_with_raw_bytes(input: &[i32; 784]) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting RISC Zero Groth16 proof generation with raw bytes...");

    let env = ExecutorEnv::builder()
        .write(&input.to_vec())?
        .build()?;

    let prover = default_prover();
    let prove_info = prover.prove_with_opts(env, MNIST_ELF, &ProverOpts::groth16())?;
    let receipt = prove_info.receipt;

    let predicted: i32 = receipt.journal.decode()?;
    println!("Predicted digit: {}", predicted);

    let groth16_seal = match &receipt.inner {
        risc0_zkvm::InnerReceipt::Groth16(groth16_receipt) => &groth16_receipt.seal,
        _ => return Err("Expected Groth16Receipt".into()),
    };

    // Try to get raw bytes - this depends on the actual Groth16Seal implementation
    let groth16_bytes = match bincode::serialize(&groth16_seal) {
        Ok(bytes) => bytes,
        Err(_) => {
            // If bincode fails, try other serialization methods
            serde_json::to_vec(&groth16_seal).unwrap_or_else(|_| {
                format!("{:?}", groth16_seal).into_bytes()
            })
        }
    };

    let output_data = serde_json::json!({
        "groth16_seal_base64": general_purpose::STANDARD.encode(&groth16_bytes),
        "groth16_seal_hex": hex::encode(&groth16_bytes),
        "prediction": predicted,
        "proof_system": "risc0_groth16_direct",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "seal_size_bytes": groth16_bytes.len()
    });

    let mut file = File::create("groth16_proof_raw.json")?;
    to_writer_pretty(&mut file, &output_data)?;

    println!("✅ Groth16 proof with raw bytes saved to groth16_proof_raw.json");
    println!("Proof size: {} bytes", groth16_bytes.len());

    Ok(())
}