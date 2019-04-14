use libc::c_void;
use std::ffi::{CStr, CString};

use crate::error::*;
use crate::filter::*;

pub struct MagickWand {
    pub(crate) wand: *mut magickwand_sys::MagickWand,
}

impl MagickWand {
    pub fn new() -> Result<Self, MagickError> {
        unsafe {
            let wand = magickwand_sys::NewMagickWand();
            if wand.is_null() {
                Err(MagickError::NullPointerResult("NewMagickWand"))
            } else {
                Ok(MagickWand { wand })
            }
        }
    }

    pub fn clone(&self) -> Result<Self, MagickError> {
        unsafe {
            let cloned_wand = magickwand_sys::CloneMagickWand(self.wand);
            if cloned_wand.is_null() {
                Err(MagickError::NullPointerResult("CloneMagickWand"))
            } else {
                Ok(MagickWand { wand: cloned_wand })
            }
        }
    }

    pub fn read_image(mut self, path: &str) -> Result<Self, MagickError> {
        unsafe {
            let raw_path = CString::new(path)?;
            let read_result = magickwand_sys::MagickReadImage(self.wand, raw_path.as_ptr());
            if read_result == magickwand_sys::MagickBooleanType_MagickTrue {
                Ok(self)
            } else {
                Err(self.get_magick_exception())
            }
        }
    }

    pub fn get_image_width(&self) -> usize {
        unsafe { magickwand_sys::MagickGetImageWidth(self.wand) }
    }

    pub fn get_image_height(&self) -> usize {
        unsafe { magickwand_sys::MagickGetImageHeight(self.wand) }
    }

    pub fn get_image_size(&self) -> (usize, usize) {
        (self.get_image_width(), self.get_image_height())
    }

    pub fn resize_image(
        mut self,
        width: usize,
        height: usize,
        filter: FilterType,
    ) -> Result<Self, MagickError> {
        unsafe {
            let resize_result =
                magickwand_sys::MagickResizeImage(self.wand, width, height, filter.into());
            if resize_result == magickwand_sys::MagickBooleanType_MagickTrue {
                Ok(self)
            } else {
                Err(self.get_magick_exception())
            }
        }
    }

    pub fn set_image_compression(mut self, quality: usize) -> Result<Self, MagickError> {
        unsafe {
            let change_result =
                magickwand_sys::MagickSetImageCompressionQuality(self.wand, quality);
            if change_result == magickwand_sys::MagickBooleanType_MagickTrue {
                Ok(self)
            } else {
                Err(self.get_magick_exception())
            }
        }
    }

    pub fn write_image(mut self, path: &str) -> Result<Self, MagickError> {
        unsafe {
            let path_raw = CString::new(path)?;
            let write_result =
                magickwand_sys::MagickWriteImage(self.wand, path_raw.as_ptr());
            if write_result == magickwand_sys::MagickBooleanType_MagickTrue {
                Ok(self)
            } else {
                Err(self.get_magick_exception())
            }
        }

    }

    unsafe fn get_magick_exception(&mut self) -> MagickError {
        let mut exception_type: magickwand_sys::ExceptionType =
            magickwand_sys::ExceptionType_UndefinedException;
        let exception_description_raw =
            magickwand_sys::MagickGetException(self.wand, &mut exception_type);
        if exception_description_raw.is_null() {
            return MagickError::NullPointerResult("MagickGetException");
        }

        let exception = CStr::from_ptr(exception_description_raw)
            .to_str()
            .map(|exception_description| {
                MagickError::MagickException(exception_type, exception_description.to_string())
            })
            .unwrap_or_else(MagickError::from);

        magickwand_sys::MagickRelinquishMemory(exception_description_raw as *mut c_void);

        exception
    }
}

impl std::ops::Drop for MagickWand {
    fn drop(&mut self) {
        unsafe {
            magickwand_sys::DestroyMagickWand(self.wand);
        }
    }
}
