use std::{
    ffi::{CString, OsStr},
    os::unix::ffi::OsStrExt,
};

/// Copy an `OsStr` into a fixed-size null-terminated `[i8; N]` C buffer.
/// Truncates silently if `src` is longer than `N - 1` bytes.
pub(crate) fn copy_osstr<const N: usize>(dst: &mut [i8; N], src: &OsStr) {
    let src = src.as_bytes();
    let n = src.len().min(N - 1);
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr() as *const i8, dst.as_mut_ptr(), n);
        dst[n] = 0;
    }
}

/// Owned array of C strings with a stable pointer list for FFI calls.
///
/// `as_ptr` holds pointers into each `CString`'s owned string buffer. Moving this
/// struct retains the validity of the pointers to the heap-allocated strings.
pub(crate) struct CStringArray {
    _strings: Vec<CString>,
    ptrs: Vec<*const i8>,
}

impl CStringArray {
    /// Build from a slice of paths. Returns the offending path if one contains a NUL byte.
    pub(crate) fn try_new<T: AsRef<OsStr>>(paths: &[T]) -> Result<Self, &OsStr> {
        let strings = paths
            .iter()
            .map(|p| {
                let os = p.as_ref();
                CString::new(os.as_bytes()).map_err(|_| os)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let ptrs = strings.iter().map(|s| s.as_ptr()).collect();
        Ok(Self { _strings: strings, ptrs })
    }

    /// Build from a single path. Returns the offending path if it contains a NUL byte.
    pub(crate) fn try_single(path: &OsStr) -> Result<Self, &OsStr> {
        let string = CString::new(path.as_bytes()).map_err(|_| path)?;
        Ok(Self {
            ptrs: vec![string.as_ptr()],
            _strings: vec![string],
        })
    }

    /// Pointer to the first string. Panics if empty.
    pub(crate) fn first(&self) -> *const i8 {
        self.ptrs[0]
    }

    /// Mutable pointer to the start of the pointer array, for passing to C as `char **`.
    pub(crate) fn as_mut_ptr(&mut self) -> *mut *const i8 {
        self.ptrs.as_mut_ptr()
    }

    /// Number of strings in the array.
    pub(crate) fn len(&self) -> usize {
        self.ptrs.len()
    }
}
