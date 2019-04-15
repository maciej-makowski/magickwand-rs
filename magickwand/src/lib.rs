#![macro_use]
extern crate ctor;
extern crate libc;
extern crate magickwand_sys;

use ctor::*;
use libc::{c_void, size_t};
use std::ffi::CString;

use magickwand_sys::*;

mod colorspace;
pub use crate::colorspace::ColorspaceType;

mod compression;
pub use crate::compression::CompressionType;

mod filter;
pub use crate::filter::FilterType;

mod error;
pub use crate::error::MagickError;

mod resource;
pub use crate::resource::ResourceType;

mod wand;
pub use crate::wand::MagickWand;



mod utils;
use crate::utils::from_borrowed_magick_string;


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
            let format = from_borrowed_magick_string(*(formats.offset(i)))?;
            result.push(format);
        }
        MagickRelinquishMemory(formats as *mut c_void);

        Ok(result)
    }
}

pub fn get_copyright() -> Result<String, MagickError> {
    unsafe { from_borrowed_magick_string(MagickGetCopyright()) }
}

pub fn get_version() -> Result<String, MagickError> {
    unsafe {
        let mut _ignored: size_t = 0;
        from_borrowed_magick_string(MagickGetVersion(&mut _ignored))
    }
}

pub fn get_version_number() -> usize {
    unsafe {
        let mut version: size_t = 0;
        MagickGetVersion(&mut version);
        version
    }
}

pub fn get_package_name() -> Result<String, MagickError> {
    unsafe { from_borrowed_magick_string(MagickGetPackageName()) }
}

pub fn get_quantum_depth() -> Result<String, MagickError> {
    unsafe { 
        let mut _ignored: size_t = 0;
        from_borrowed_magick_string(MagickGetQuantumDepth(&mut _ignored))
    }
}

pub fn get_quantum_depth_size() -> usize {
    unsafe { 
        let mut size: size_t = 0;
        from_borrowed_magick_string(MagickGetQuantumDepth(&mut size));
        size
    }
}

pub fn get_quantum_range() -> Result<String, MagickError> {
    unsafe { 
        let mut _ignored: size_t = 0;
        from_borrowed_magick_string(MagickGetQuantumRange(&mut _ignored))
    }
}

pub fn get_quantum_range_size() -> usize {
    unsafe { 
        let mut size: size_t = 0;
        from_borrowed_magick_string(MagickGetQuantumRange(&mut size));
        size
    }
}