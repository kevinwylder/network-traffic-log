import numpy as np
import dataloader

a = np.zeros((7, 24 * 60),  dtype=np.float32)
dataloader.rasterize_week(a, "../data/2022-10-10")
print(a)
