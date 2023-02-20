import numpy as np
import os
import dataloader

data_dir = os.getenv("DATA_PATH")
assert data_dir, "DATA_PATH env var not set!"

a = np.zeros((7, 24 * 60),  dtype=np.float32)
dataloader.render_py(a, os.path.join(data_dir, "2022-10-10"))
print(a)
