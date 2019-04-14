use std::env;

pub(crate) fn find_magickwand_include() -> String {
    env::var("MAGICKWAND_INCLUDE").unwrap_or("/usr/include/ImageMagick-7".to_string())
}

pub(crate) fn find_magickwand_lib() -> String {
    env::var("MAGICKWAND_LIB").unwrap_or("MagickWand-7.Q16HDRI".to_string())
}

pub(crate) fn find_magickwand_search_path() -> Option<String> {
    env::var("MAGICKWAND_PATH").ok()
}
