#![allow(non_upper_case_globals)]

use self::ColorspaceType::*;
use magickwand_sys::*;

#[derive(Debug, PartialEq)]
pub enum ColorspaceType {
    Undefined,
    CMY,
    CMYK,
    Gray,
    HCL,
    HCLp,
    HSB,
    HSI,
    HSL,
    HSV,
    HWB,
    Lab,
    LCH,
    LCHab,
    LCHuv,
    Log,
    LMS,
    Luv,
    OHTA,
    Rec601YCbCr,
    Rec709YCbCr,
    RGB,
    ScRGB,
    SRGB,
    Transparent,
    XYy,
    XYZ,
    YCbCr,
    YCC,
    YDbDr,
    YIQ,
    YPbPr,
    YUV,
    LinearGray,
}

impl From<magickwand_sys::ColorspaceType> for ColorspaceType {
    fn from(value: magickwand_sys::ColorspaceType) -> ColorspaceType {
        match value {
            ColorspaceType_CMYColorspace => CMY,
            ColorspaceType_CMYKColorspace => CMYK,
            ColorspaceType_GRAYColorspace => Gray,
            ColorspaceType_HCLColorspace => HCL,
            ColorspaceType_HCLpColorspace => HCLp,
            ColorspaceType_HSBColorspace => HSB,
            ColorspaceType_HSIColorspace => HSI,
            ColorspaceType_HSLColorspace => HSL,
            ColorspaceType_HSVColorspace => HSV,
            ColorspaceType_HWBColorspace => HWB,
            ColorspaceType_LabColorspace => Lab,
            ColorspaceType_LCHColorspace => LCH,
            ColorspaceType_LCHabColorspace => LCHab,
            ColorspaceType_LCHuvColorspace => LCHuv,
            ColorspaceType_LogColorspace => Log,
            ColorspaceType_LMSColorspace => LMS,
            ColorspaceType_LuvColorspace => Luv,
            ColorspaceType_OHTAColorspace => OHTA,
            ColorspaceType_Rec601YCbCrColorspace => Rec601YCbCr,
            ColorspaceType_Rec709YCbCrColorspace => Rec709YCbCr,
            ColorspaceType_RGBColorspace => RGB,
            ColorspaceType_scRGBColorspace => ScRGB,
            ColorspaceType_sRGBColorspace => SRGB,
            ColorspaceType_TransparentColorspace => Transparent,
            ColorspaceType_xyYColorspace => XYy,
            ColorspaceType_XYZColorspace => XYZ,
            ColorspaceType_YCbCrColorspace => YCbCr,
            ColorspaceType_YCCColorspace => YCC,
            ColorspaceType_YDbDrColorspace => YDbDr,
            ColorspaceType_YIQColorspace => YIQ,
            ColorspaceType_YPbPrColorspace => YPbPr,
            ColorspaceType_YUVColorspace => YUV,
            ColorspaceType_LinearGRAYColorspace => LinearGray,
            _ => Undefined,
        }
    }
}

impl Into<magickwand_sys::ColorspaceType> for ColorspaceType {
    fn into(self) -> magickwand_sys::ColorspaceType {
        match self {
            CMY => ColorspaceType_CMYColorspace,
            CMYK => ColorspaceType_CMYKColorspace,
            Gray => ColorspaceType_GRAYColorspace,
            HCL => ColorspaceType_HCLColorspace,
            HCLp => ColorspaceType_HCLpColorspace,
            HSB => ColorspaceType_HSBColorspace,
            HSI => ColorspaceType_HSIColorspace,
            HSL => ColorspaceType_HSLColorspace,
            HSV => ColorspaceType_HSVColorspace,
            HWB => ColorspaceType_HWBColorspace,
            Lab => ColorspaceType_LabColorspace,
            LCH => ColorspaceType_LCHColorspace,
            LCHab => ColorspaceType_LCHabColorspace,
            LCHuv => ColorspaceType_LCHuvColorspace,
            Log => ColorspaceType_LogColorspace,
            LMS => ColorspaceType_LMSColorspace,
            Luv => ColorspaceType_LuvColorspace,
            OHTA => ColorspaceType_OHTAColorspace,
            Rec601YCbCr => ColorspaceType_Rec601YCbCrColorspace,
            Rec709YCbCr => ColorspaceType_Rec709YCbCrColorspace,
            RGB => ColorspaceType_RGBColorspace,
            ScRGB => ColorspaceType_scRGBColorspace,
            SRGB => ColorspaceType_sRGBColorspace,
            Transparent => ColorspaceType_TransparentColorspace,
            XYy => ColorspaceType_xyYColorspace,
            XYZ => ColorspaceType_XYZColorspace,
            YCbCr => ColorspaceType_YCbCrColorspace,
            YCC => ColorspaceType_YCCColorspace,
            YDbDr => ColorspaceType_YDbDrColorspace,
            YIQ => ColorspaceType_YIQColorspace,
            YPbPr => ColorspaceType_YPbPrColorspace,
            YUV => ColorspaceType_YUVColorspace,
            LinearGray => ColorspaceType_LinearGRAYColorspace,
            _ => ColorspaceType_UndefinedColorspace,
        }
    }
}
