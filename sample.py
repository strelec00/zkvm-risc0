"""
import numpy as np

# Prazna 28x28 slika
img = np.zeros((28, 28), dtype=np.int32)

# Grubo crtanje broja 2
img[5:8, 8:20] = 1          # gornja crta
img[8:14, 19:21] = 1        # gornja desna
img[13:15, 8:20] = 1        # srednja crta
img[15:21, 7:9] = 1         # donja lijeva
img[20:23, 8:20] = 1        # donja crta

# Spremi kao .npy za testiranje
np.save("sample_input.npy", img.flatten())

# Generiraj Rust array za SAMPLE
with open("sample_input.rs", "w") as f:
    f.write("pub const SAMPLE: [i32; 784] = [\n")
    flat = img.flatten()
    for i in range(0, len(flat), 28):
        f.write("    " + ", ".join(map(str, flat[i:i+28])) + ",\n")
    f.write("];\n")


import numpy as np

# Prazna 28x28 slika
img = np.zeros((28, 28), dtype=np.int32)

# Grubo crtanje broja 9
img[5:8, 8:20] = 1           # gornja crta
img[8:14, 19:21] = 1         # gornja desna
img[13:15, 8:20] = 1         # srednja crta
img[8:15, 7:9] = 1           # gornja lijeva
img[15:22, 19:21] = 1        # donja desna
img[20:23, 8:20] = 1         # donja crta

# Spremi kao .npy za testiranje
np.save("sample_input.npy", img.flatten())

# Generiraj Rust array za SAMPLE
with open("sample_input.rs", "w") as f:
    f.write("pub const SAMPLE: [i32; 784] = [\n")
    flat = img.flatten()
    for i in range(0, len(flat), 28):
        f.write("    " + ", ".join(map(str, flat[i:i+28])) + ",\n")
    f.write("];\n")


import numpy as np

# Prazna 28x28 slika
img = np.zeros((28, 28), dtype=np.int32)

# Grubo crtanje broja 4
img[5:18, 18:20] = 1         # desna okomita crta
img[12:14, 8:20] = 1         # vodoravna crta u sredini
img[5:14, 8:10] = 1          # lijeva gornja okomita crta

# Spremi kao .npy za testiranje
np.save("sample_input.npy", img.flatten())

# Generiraj Rust array za SAMPLE
with open("sample_input.rs", "w") as f:
    f.write("pub const SAMPLE: [i32; 784] = [\n")
    flat = img.flatten()
    for i in range(0, len(flat), 28):
        f.write("    " + ", ".join(map(str, flat[i:i+28])) + ",\n")
    f.write("];\n")

"""
import numpy as np

# Prazna 28x28 slika
img = np.zeros((28, 28), dtype=np.int32)

# Grubo crtanje broja 1
img[5:23, 13:15] = 1         # središnja okomita crta
img[5:8, 12:16] = 1          # gornji vodoravni dio
img[22:24, 12:16] = 1        # donji vodoravni dio (baza)

# Spremi kao .npy
np.save("sample_input.npy", img.flatten())

# Generiraj Rust array
with open("sample_input.rs", "w") as f:
    f.write("pub const SAMPLE: [i32; 784] = [\n")
    flat = img.flatten()
    for i in range(0, len(flat), 28):
        f.write("    " + ", ".join(map(str, flat[i:i+28])) + ",\n")
    f.write("];\n")
