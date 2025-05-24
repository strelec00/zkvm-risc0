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

mod sample {
    include!("input/sample_input.rs"); // const SAMPLE: [i32; 784]
}

use host::run_inference;
use mnist_methods::MNIST_ID;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let (receipt, predicted_digit) = run_inference(&sample::SAMPLE);
    receipt.verify(MNIST_ID).expect("Verification failed.");

    println!("Proof is valid and inference is done!");
    println!("Prepoznati broj je: {}", predicted_digit);
}