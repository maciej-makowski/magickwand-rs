use std::ffi::CString;

use crate::colorspace::ColorspaceType;
use crate::error::*;
use crate::filter::FilterType;
use crate::compression::CompressionType;
use crate::utils::from_owned_magick_string;

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

    pub fn write_image(mut self, path: &str) -> Result<Self, MagickError> {
        unsafe {
            let path_raw = CString::new(path)?;
            let write_result = magickwand_sys::MagickWriteImage(self.wand, path_raw.as_ptr());
            if write_result == magickwand_sys::MagickBooleanType_MagickTrue {
                Ok(self)
            } else {
                Err(self.get_magick_exception())
            }
        }
    }

    pub fn get_compression(&self) -> CompressionType {
        unsafe { magickwand_sys::MagickGetCompression(self.wand).into() }
    }

    pub fn set_compression(mut self, compression: CompressionType) -> Result<Self, MagickError> {
        unsafe {
            let set_result = magickwand_sys::MagickSetCompression(self.wand, compression.into());
            if set_result == magickwand_sys::MagickBooleanType_MagickTrue {
                Ok(self)
            } else {
                Err(self.get_magick_exception())
            }
        }
    }

    pub fn get_compression_quality(&self) -> usize {
        unsafe { magickwand_sys::MagickGetCompressionQuality(self.wand) }
    }

    pub fn set_image_compression_quality(mut self, quality: usize) -> Result<Self, MagickError> {
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

    pub fn has_antialias(&self) -> bool {
        unsafe {
            magickwand_sys::MagickGetAntialias(self.wand)
                == magickwand_sys::MagickBooleanType_MagickTrue
        }
    }

    pub fn set_antialias(mut self, enabled: bool) -> Result<MagickWand, MagickError> {
        unsafe {
            let set_result = magickwand_sys::MagickSetAntialias(
                self.wand,
                if enabled {
                    magickwand_sys::MagickBooleanType_MagickTrue
                } else {
                    magickwand_sys::MagickBooleanType_MagickFalse
                },
            );
            if set_result == magickwand_sys::MagickBooleanType_MagickTrue {
                Ok(self)
            } else {
                Err(self.get_magick_exception())
            }
        }
    }

    pub fn get_colorspace(&self) -> ColorspaceType {
        unsafe { magickwand_sys::MagickGetColorspace(self.wand).into() }
    }

    pub fn set_colorspace(mut self, colorspace: ColorspaceType) -> Result<MagickWand, MagickError> {
        unsafe {
            let set_result = magickwand_sys::MagickSetColorspace(self.wand, colorspace.into());
            if set_result == magickwand_sys::MagickBooleanType_MagickTrue {
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

        from_owned_magick_string(exception_description_raw)
            .map(|exception_description| {
                MagickError::MagickException(exception_type, exception_description.to_string())
            })
            .unwrap_or_else(std::convert::identity)
    }
}

impl std::ops::Drop for MagickWand {
    fn drop(&mut self) {
        unsafe {
            magickwand_sys::DestroyMagickWand(self.wand);
        }
    }
}
