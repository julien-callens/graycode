"""
Python facade for the native extension.

Re-exports everything from the compiled module so that static analysers
see the names *and* users get friendly runtime help().
"""
from importlib import import_module
from typing import TYPE_CHECKING

_native = import_module("graycode")

bit_plane = _native.bit_plane

if TYPE_CHECKING:
    import numpy as _np
    from typing import Optional as _Opt
    def bit_plane(
            bit: int, *, width: _Opt[int] = ..., height: _Opt[int] = ...
    ) -> _np.ndarray: ...
