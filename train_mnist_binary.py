# train_mnist_binary.py
import torch
import torch.nn as nn
import torchvision
import numpy as np

# 1. Dataset (binariziran MNIST)
transform = torchvision.transforms.Compose([
    torchvision.transforms.ToTensor(),
    torchvision.transforms.Lambda(lambda x: (x > 0.5).float())  # binarizacija
])

train_set = torchvision.datasets.MNIST(root='./data', train=True, transform=transform, download=True)
train_loader = torch.utils.data.DataLoader(train_set, batch_size=64, shuffle=True)

# 2. Model: 784 -> 64 -> 10
class MLP(nn.Module):
    def __init__(self):
        super().__init__()
        self.net = nn.Sequential(
            nn.Flatten(),
            nn.Linear(784, 64),
            nn.ReLU(),
            nn.Linear(64, 10)
        )

    def forward(self, x):
        return self.net(x)

model = MLP()
loss_fn = nn.CrossEntropyLoss()
optimizer = torch.optim.Adam(model.parameters(), lr=1e-3)

# 3. Trening (par epoha za brzinu)
for epoch in range(3):
    for x, y in train_loader:
        logits = model(x)
        loss = loss_fn(logits, y)

        optimizer.zero_grad()
        loss.backward()
        optimizer.step()

# 4. Export težina (u fixed-point i32)
def to_fixed(tensor, scale=1000):
    return (tensor.detach().numpy() * scale).astype(np.int32)

np.save("W1.npy", to_fixed(model.net[1].weight.T))
np.save("B1.npy", to_fixed(model.net[1].bias))
np.save("W2.npy", to_fixed(model.net[3].weight.T))
np.save("B2.npy", to_fixed(model.net[3].bias))