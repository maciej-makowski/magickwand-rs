use libc::{c_char, c_void};
use std::ffi::CStr;

use magickwand_sys::MagickRelinquishMemory;

use crate::error::MagickError;

pub(crate) unsafe fn from_borrowed_magick_string(
    magick_string: *const c_char,
) -> Result<String, MagickError> {
    if magick_string.is_null() {
        Err(MagickError::NullPointerResult(
            "from_borrowed_magick_string",
        ))
    } else {
        let rust_string = CStr::from_ptr(magick_string).to_str()?.to_string();

        Ok(rust_string)
    }
}

pub(crate) unsafe fn from_owned_magick_string(
    magick_string: *mut c_char,
) -> Result<String, MagickError> {
    if magick_string.is_null() {
        Err(MagickError::NullPointerResult("from_owned_magick_string"))
    } else {
        let rust_string = CStr::from_ptr(magick_string).to_str()?.to_string();

        MagickRelinquishMemory(magick_string as *mut c_void);

        Ok(rust_string)
    }
}
