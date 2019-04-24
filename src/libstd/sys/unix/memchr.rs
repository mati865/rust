// Original implementation taken from rust-memchr.
// Copyright 2015 Andrew Gallant, bluss and Nicolas Koch

pub fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    let p = unsafe {
        libc::memchr(
            haystack.as_ptr() as *const libc::c_void,
            needle as libc::c_int,
            haystack.len())
    };
    if p.is_null() {
        None
    } else {
        Some(p as usize - (haystack.as_ptr() as usize))
    }
}

pub fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {

    // FIXME: relibc missing symbol
    #[cfg(all(target_os = "linux", not(target_env = "relibc")))]
    fn memrchr_specific(needle: u8, haystack: &[u8]) -> Option<usize> {
        // GNU's memrchr() will - unlike memchr() - error if haystack is empty.
        if haystack.is_empty() {return None}
        let p = unsafe {
            libc::memrchr(
                haystack.as_ptr() as *const libc::c_void,
                needle as libc::c_int,
                haystack.len())
        };
        if p.is_null() {
            None
        } else {
            Some(p as usize - (haystack.as_ptr() as usize))
        }
    }

    // FIXME: relibc missing symbol
    #[cfg(any(not(target_os = "linux"), target_env = "relibc"))]
    fn memrchr_specific(needle: u8, haystack: &[u8]) -> Option<usize> {
        core::slice::memchr::memrchr(needle, haystack)
    }

    memrchr_specific(needle, haystack)
}
