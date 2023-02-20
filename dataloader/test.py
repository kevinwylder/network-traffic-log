import numpy as np
import os
import dataloader

data_dir = os.getenv("DATA_PATH")
assert data_dir, "DATA_PATH env var not set!"
data_file =  os.path.join(data_dir, "2022-10-10")

a = np.zeros((7, 24 * 60),  dtype=np.float32)
dataloader.volume_data(a, data_file)
print(a)

a = np.zeros((7, 24 * 60),  dtype=np.float32)
dataloader.frequency_data(a, data_file, 1_000_000)
print(a)