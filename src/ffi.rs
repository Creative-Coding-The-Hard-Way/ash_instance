use std::{
    ffi::{c_char, CString},
    string::FromUtf8Error,
};

/// Build a vector of CStrings and a matching vector of pointers to those
/// strings.
///
/// # Safety
///
/// Unsafe because:
///   - The pointers are only valid so long as the returned strings are not
///     dropped or modified.
pub unsafe fn to_os_ptrs(
    strings: &[String],
) -> (Vec<CString>, Vec<*const c_char>) {
    let cstrings = strings
        .iter()
        .cloned()
        .map(|str| CString::new(str).unwrap())
        .collect::<Vec<CString>>();
    let ptrs = cstrings
        .iter()
        .map(|cstr| cstr.as_ptr())
        .collect::<Vec<*const c_char>>();
    (cstrings, ptrs)
}

/// Build a String from a slice of utf-8 bytes.
///
/// Automatically truncates any nul bytes from the end of the string (these
/// often show up in buffers used for names of Vulkan
/// objects/layers/extensions).
pub fn string_from_i8(bytes: &[i8]) -> Result<String, FromUtf8Error> {
    String::from_utf8(
        bytes
            .iter()
            .filter(|&c| *c != 0)
            .map(|c| *c as u8)
            .collect(),
    )
}
