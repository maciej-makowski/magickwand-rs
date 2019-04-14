#![macro_use]
extern crate ctor;
extern crate libc;
extern crate magickwand_sys;

use ctor::*;
use libc::{c_void, size_t};
use std::ffi::{CStr, CString};

use magickwand_sys::*;

mod error;
pub use crate::error::MagickError;

mod wand;
pub use crate::wand::MagickWand;

mod filter;
pub use crate::filter::FilterType;

#[ctor]
unsafe fn magickwand_initialize() {
    MagickWandGenesis();
}

#[dtor]
unsafe fn magickwand_destory() {
    MagickWandTerminus();
}

pub fn query_formats(filter: &str) -> Result<Vec<String>, MagickError> {
    unsafe {
        let mut size: size_t = 0;

        let raw_filter = CString::new(filter)?;
        let formats = MagickQueryFormats(raw_filter.as_ptr(), &mut size);

        if formats.is_null() {
            return Err(MagickError::NullPointerResult("MagickQueryFormats"));
        }

        let mut result: Vec<String> = Vec::with_capacity(size);
        for i in 0..size as isize {
            let format_raw = formats.offset(i);
            let format_str = CStr::from_ptr(*format_raw);
            let format = format_str.to_str()?.to_string();
            result.push(format);
        }
        MagickRelinquishMemory(formats as *mut c_void);

        Ok(result)
    }
}
