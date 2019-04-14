use magickwand_sys::ExceptionType;

#[derive(Debug)]
pub enum MagickError {
    NulError(std::ffi::NulError),
    Utf8Error(std::str::Utf8Error),
    NullPointerResult(&'static str),
    MagickException(ExceptionType, String),
}

impl std::error::Error for MagickError {}

impl std::fmt::Display for MagickError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::convert::From<std::ffi::NulError> for MagickError {
    fn from(err: std::ffi::NulError) -> MagickError {
        MagickError::NulError(err)
    }
}

impl std::convert::From<std::str::Utf8Error> for MagickError {
    fn from(err: std::str::Utf8Error) -> MagickError {
        MagickError::Utf8Error(err)
    }
}
