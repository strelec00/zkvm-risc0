// guest/src/bin/mnist.rs

use risc0_zkvm::guest::env;

mod weights {
    include!("weights/W1.incl.rs");
    include!("weights/B1.incl.rs");
    include!("weights/W2.incl.rs");
    include!("weights/B2.incl.rs");
}

fn relu(x: i32) -> i32 {
    x.max(0)
}

fn main() {
    let input_vec: Vec<i32> = env::read();
    println!("Prva tri inputa: {}, {}, {}", input_vec[0], input_vec[1], input_vec[2]);

    let input: [i32; 784] = input_vec.try_into().expect("Expected input of length 784");

    // layer 1
    let mut h1 = [0i32; 64];
    for i in 0..64 {
        let mut sum = weights::B1[i];
        for j in 0..784 {
            sum += weights::W1[j][i] * input[j];        }
        h1[i] = relu(sum);
    }

    // layer 2
    let mut out = [0i32; 10];
    for i in 0..10 {
        let mut sum = weights::B2[i];
        for j in 0..64 {
            sum += weights::W2[j][i] * h1[j] ;
        }
        out[i] = sum;
    }

    // argmax
    let (mut predicted_digit, mut max_val) = (0, out[0]);
    for i in 1..10 {
        if out[i] > max_val {
            max_val = out[i];
            predicted_digit = i;
        }
    }

    env::commit(&predicted_digit
    );
}