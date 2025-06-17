import graycode, numpy as np, time
import matplotlib.pyplot as plt

if __name__ == '__main__':
    t0 = time.perf_counter()
    img = graycode.bit_plane(3)
    print(img.shape, img.dtype, f"{(time.perf_counter()-t0)*1e3:.1f} ms")
    plt.imshow(img, cmap="gray"); plt.show()