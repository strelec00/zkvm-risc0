# Popravljeni convert_to_rust.py
import numpy as np
import os

def to_rust_array(name, arr):
    flat = arr.reshape(arr.shape[0], -1)
    lines = [f"    [{', '.join(map(str, row))}]," for row in flat]
    return f"pub const {name}: [[i32; {flat.shape[1]}]; {flat.shape[0]}] = [\n" + "\n".join(lines) + "\n];"

def write_array(path, content):
    with open(path, "w") as f:
        f.write(content)

os.makedirs("weights", exist_ok=True)

W1 = np.load("W1.npy")
B1 = np.load("B1.npy")
W2 = np.load("W2.npy")
B2 = np.load("B2.npy")

write_array("weights/W1.incl.rs", to_rust_array("W1", W1))
write_array("weights/B1.incl.rs", f"pub const B1: [i32; 64] = [{', '.join(map(str, B1))}];")
write_array("weights/W2.incl.rs", to_rust_array("W2", W2))
write_array("weights/B2.incl.rs", f"pub const B2: [i32; 10] = [{', '.join(map(str, B2))}];")