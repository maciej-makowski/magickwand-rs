#![allow(non_upper_case_globals)]

use crate::compression::CompressionType::*;
use magickwand_sys::*;

#[derive(Debug, PartialEq)]
pub enum CompressionType {
    Undefined,
    B44A,
    B44,
    BZip,
    DXT1,
    DXT3,
    DXT5,
    Fax,
    Group4,
    JBIG1,
    JBIG2,
    JPEG2000,
    JPEG,
    LosslessJPEG,
    LZMA,
    LZW,
    NoCompression,
    Piz,
    Pxr24,
    RLE,
    Zip,
    ZipS,
    Zstd,
    WebP
}

impl From<magickwand_sys::CompressionType> for CompressionType {
    fn from(value: magickwand_sys::CompressionType) -> CompressionType {
        match value {
            CompressionType_B44ACompression => B44A,
            CompressionType_B44Compression => B44,
            CompressionType_BZipCompression => BZip,
            CompressionType_DXT1Compression => DXT1,
            CompressionType_DXT3Compression => DXT3,
            CompressionType_DXT5Compression => DXT5,
            CompressionType_FaxCompression => Fax,
            CompressionType_Group4Compression => Group4,
            CompressionType_JBIG1Compression => JBIG1,
            CompressionType_JBIG2Compression => JBIG2,
            CompressionType_JPEG2000Compression => JPEG2000,
            CompressionType_JPEGCompression => JPEG,
            CompressionType_LosslessJPEGCompression => LosslessJPEG,
            CompressionType_LZMACompression => LZMA,
            CompressionType_LZWCompression => LZW,
            CompressionType_NoCompression => NoCompression,
            CompressionType_PizCompression => Piz,
            CompressionType_Pxr24Compression => Pxr24,
            CompressionType_RLECompression => RLE,
            CompressionType_ZipCompression => Zip,
            CompressionType_ZipSCompression => ZipS,
            CompressionType_ZstdCompression => Zstd,
            CompressionType_WebPCompression => WebP,
            _ => Undefined
        }
    }
}

impl Into<magickwand_sys::CompressionType> for CompressionType {
    fn into(self) -> magickwand_sys::CompressionType {
        match self {
            B44A => CompressionType_B44ACompression,
            B44 => CompressionType_B44Compression,
            BZip => CompressionType_BZipCompression,
            DXT1 => CompressionType_DXT1Compression,
            DXT3 => CompressionType_DXT3Compression,
            DXT5 => CompressionType_DXT5Compression,
            Fax => CompressionType_FaxCompression,
            Group4 => CompressionType_Group4Compression,
            JBIG1 => CompressionType_JBIG1Compression,
            JBIG2 => CompressionType_JBIG2Compression,
            JPEG2000 => CompressionType_JPEG2000Compression,
            JPEG => CompressionType_JPEGCompression,
            LosslessJPEG => CompressionType_LosslessJPEGCompression,
            LZMA => CompressionType_LZMACompression,
            LZW => CompressionType_LZWCompression,
            NoCompression => CompressionType_NoCompression,
            Piz => CompressionType_PizCompression,
            Pxr24 => CompressionType_Pxr24Compression,
            RLE => CompressionType_RLECompression,
            Zip => CompressionType_ZipCompression,
            ZipS => CompressionType_ZipSCompression,
            Zstd => CompressionType_ZstdCompression,
            WebP => CompressionType_WebPCompression,
            _ => CompressionType_UndefinedCompression
        }
    }
}
