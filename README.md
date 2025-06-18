# graycode

Fast Gray-code bit-plane generator exposed as a NumPy-friendly Python module, written in Rust + PyO3.

## Install (local dev)

```bash
pip install maturin
maturin develop --release            # add --features parallel for multi-core speed
```

## Use

```python
import graycode                      # compiled extension
img = graycode.bit_plane(3, width=640, height=480)
```

*Returns a `numpy.ndarray` (`uint8`, H × W) filled with 0/255.*
