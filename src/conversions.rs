// https://github.com/PyO3/pyo3/blob/16ee22c7ccd7dab66242ff95da20dd71fd7684b2/src/conversions/num_bigint.rs

use ethers::prelude::*;
use num_bigint::{BigInt, Sign};
use pyo3::{
    ffi, types::*, AsPyPointer, FromPyObject, IntoPy, Py, PyAny, PyErr, PyObject, PyResult, Python,
    ToPyObject,
};
use std::os::raw::{c_int, c_uchar};

pub fn u256_to_bigint(u: U256) -> BigInt {
    let mut bytes = [0u8; 32];
    u.to_big_endian(&mut bytes);
    BigInt::from_bytes_be(Sign::Plus, &bytes)
}
pub struct PyU256(U256);

impl PyU256 {
    pub fn new(u: U256) -> PyU256 {
        PyU256(u)
    }
}

/// Returns Ok if the error code is not -1.
#[inline]
pub fn error_on_minusone(py: Python<'_>, result: c_int) -> PyResult<()> {
    if result != -1 {
        Ok(())
    } else {
        Err(PyErr::fetch(py))
    }
}

unsafe fn extract(ob: &PyLong, buffer: &mut [c_uchar], is_signed: c_int) -> PyResult<()> {
    error_on_minusone(
        ob.py(),
        ffi::_PyLong_AsByteArray(
            ob.as_ptr() as *mut ffi::PyLongObject,
            buffer.as_mut_ptr(),
            buffer.len(),
            1,
            is_signed,
        ),
    )
}

macro_rules! bigint_conversion {
    ($rust_ty: ty, $is_signed: expr, $to_bytes: path, $from_bytes: path) => {
        impl ToPyObject for $rust_ty {
            fn to_object(&self, py: Python<'_>) -> PyObject {
                unsafe {
                    let bytes = $to_bytes(self);
                    let obj = ffi::_PyLong_FromByteArray(
                        bytes.as_ptr() as *const c_uchar,
                        bytes.len(),
                        1,
                        $is_signed,
                    );
                    PyObject::from_owned_ptr(py, obj)
                }
            }
        }

        impl IntoPy<PyObject> for $rust_ty {
            fn into_py(self, py: Python<'_>) -> PyObject {
                self.to_object(py)
            }
        }

        impl<'source> FromPyObject<'source> for $rust_ty {
            fn extract(ob: &'source PyAny) -> PyResult<$rust_ty> {
                let py = ob.py();
                unsafe {
                    let num: Py<PyLong> =
                        Py::from_owned_ptr_or_err(py, ffi::PyNumber_Index(ob.as_ptr()))?;
                    let n_bits = ffi::_PyLong_NumBits(num.as_ptr());
                    let n_bytes = if n_bits == (-1isize as usize) {
                        return Err(PyErr::fetch(py));
                    } else if n_bits == 0 {
                        0
                    } else {
                        (n_bits - 1 + $is_signed) / 8 + 1
                    };
                    if n_bytes <= 128 {
                        let mut buffer = [0; 128];
                        extract(num.as_ref(py), &mut buffer[..n_bytes], $is_signed)?;
                        Ok($from_bytes(&buffer[..n_bytes]))
                    } else {
                        let mut buffer = vec![0; n_bytes];
                        extract(num.as_ref(py), &mut buffer, $is_signed)?;
                        Ok($from_bytes(&buffer))
                    }
                }
            }
        }
    };
}

fn to_bytes_le(u: &PyU256) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    u.0.to_little_endian(&mut bytes);
    bytes
}

fn from_bytes_le(bytes: &[u8]) -> PyU256 {
    PyU256::new(U256::from_little_endian(bytes))
}

bigint_conversion!(PyU256, 0, to_bytes_le, from_bytes_le);
