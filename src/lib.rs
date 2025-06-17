//! graycode -- Fast Gray-code pattern generator for Python
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::Bound;
use numpy::{IntoPyArray, PyArray2};
use numpy::ndarray::{Array2, ArrayViewMut2, Axis};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Default full-HD resolution
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

#[inline]
fn gray_code(v: u32) -> u32 { v ^ (v >> 1) }

/// Fill a mutable column-major view (`view`) with the chosen Gray-code bit.
fn fill_column(mut view: ArrayViewMut2<u8>, bit: usize) {
    for (x, mut col) in view.axis_iter_mut(Axis(1)).enumerate() {
        let val = if (gray_code(x as u32) >> bit) & 1 == 1 { 255 } else { 0 };
        col.fill(val);
    }
}

/// Generate a single Gray-code bit-plane as a `numpy.ndarray` (**uint8**, H × W).
///
/// Parameters
/// ----------
/// bit : int
///     Which Gray-code bit to extract (0 = LSB).
/// width : int, optional
///     Frame width in pixels (default 1920).
/// height : int, optional
///     Frame height in pixels (default 1080).
///
/// Returns
/// -------
/// numpy.ndarray
///     2-D array whose elements are either 0 or 255.
///
/// Notes
/// -----
/// * The returned array **shares memory** with Rust (no extra copy).  
/// * When compiled with `--features parallel`, columns are split in two and
///   filled via Rayon for near-linear scaling.
///
/// -------
#[pyfunction(
    signature = (bit, width = None, height = None),
    text_signature = "(bit, /, width=None, height=None)"
)]
fn bit_plane(
    py: Python<'_>,
    bit: usize,
    width: Option<usize>,
    height: Option<usize>,
) -> PyResult<Py<PyArray2<u8>>> {
    let w = width.unwrap_or(WIDTH);
    let h = height.unwrap_or(HEIGHT);

    let mut arr: Array2<u8> = Array2::zeros((h, w));

    #[cfg(feature = "parallel")]
    {
        let (left, right) = arr.view_mut().split_at(Axis(1), w / 2);
        rayon::join(
            || fill_column(left, bit),
            || fill_column(right, bit),
        );
    }
    #[cfg(not(feature = "parallel"))]
    fill_column(arr.view_mut(), bit);

    Ok(Py::from(arr.into_pyarray(py).to_owned()))
}

#[pymodule]
fn graycode(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bit_plane, m)?)?;
    Ok(())
}
