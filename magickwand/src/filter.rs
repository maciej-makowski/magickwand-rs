use magickwand_sys::*;
use FilterType::*;

pub enum FilterType {
    UndefinedFilter,
    PointFilter,
    BoxFilter,
    TriangleFilter,
    HermiteFilter,
    HannFilter,
    HammingFilter,
    BlackmanFilter,
    GaussianFilter,
    QuadraticFilter,
    CubicFilter,
    CatromFilter,
    MitchellFilter,
    JincFilter,
    SincFilter,
    SincFastFilter,
    KaiserFilter,
    WelchFilter,
    ParzenFilter,
    BohmanFilter,
    BartlettFilter,
    LagrangeFilter,
    LanczosFilter,
    LanczosSharpFilter,
    Lanczos2Filter,
    Lanczos2SharpFilter,
    RobidouxFilter,
    RobidouxSharpFilter,
    CosineFilter,
    SplineFilter,
    LanczosRadiusFilter,
    CubicSplineFilter,
    SentinelFilter,
}

impl From<magickwand_sys::FilterType> for FilterType {
    fn from(value: magickwand_sys::FilterType) -> FilterType {
        match value {
            FilterType_PointFilter => PointFilter,
            FilterType_BoxFilter => BoxFilter,
            FilterType_TriangleFilter => TriangleFilter,
            FilterType_HermiteFilter => HermiteFilter,
            FilterType_HannFilter => HannFilter,
            FilterType_HammingFilter => HammingFilter,
            FilterType_BlackmanFilter => BlackmanFilter,
            FilterType_GaussianFilter => GaussianFilter,
            FilterType_QuadraticFilter => QuadraticFilter,
            FilterType_CubicFilter => CubicFilter,
            FilterType_CatromFilter => CatromFilter,
            FilterType_MitchellFilter => MitchellFilter,
            FilterType_JincFilter => JincFilter,
            FilterType_SincFilter => SincFilter,
            FilterType_SincFastFilter => SincFastFilter,
            FilterType_KaiserFilter => KaiserFilter,
            FilterType_WelchFilter => WelchFilter,
            FilterType_ParzenFilter => ParzenFilter,
            FilterType_BohmanFilter => BohmanFilter,
            FilterType_BartlettFilter => BartlettFilter,
            FilterType_LagrangeFilter => LagrangeFilter,
            FilterType_LanczosFilter => LanczosFilter,
            FilterType_LanczosSharpFilter => LanczosSharpFilter,
            FilterType_Lanczos2Filter => Lanczos2Filter,
            FilterType_Lanczos2SharpFilter => Lanczos2SharpFilter,
            FilterType_RobidouxFilter => RobidouxFilter,
            FilterType_RobidouxSharpFilter => RobidouxSharpFilter,
            FilterType_CosineFilter => CosineFilter,
            FilterType_SplineFilter => SplineFilter,
            FilterType_LanczosRadiusFilter => LanczosRadiusFilter,
            FilterType_CubicSplineFilter => CubicSplineFilter,
            FilterType_SentinelFilter => SentinelFilter,
            _ => UndefinedFilter,
        }
    }
}

impl Into<magickwand_sys::FilterType> for FilterType {
    fn into(self) -> magickwand_sys::FilterType {
        match self {
            PointFilter => FilterType_PointFilter,
            BoxFilter => FilterType_BoxFilter,
            TriangleFilter => FilterType_TriangleFilter,
            HermiteFilter => FilterType_HermiteFilter,
            HannFilter => FilterType_HannFilter,
            HammingFilter => FilterType_HammingFilter,
            BlackmanFilter => FilterType_BlackmanFilter,
            GaussianFilter => FilterType_GaussianFilter,
            QuadraticFilter => FilterType_QuadraticFilter,
            CubicFilter => FilterType_CubicFilter,
            CatromFilter => FilterType_CatromFilter,
            MitchellFilter => FilterType_MitchellFilter,
            JincFilter => FilterType_JincFilter,
            SincFilter => FilterType_SincFilter,
            SincFastFilter => FilterType_SincFastFilter,
            KaiserFilter => FilterType_KaiserFilter,
            WelchFilter => FilterType_WelchFilter,
            ParzenFilter => FilterType_ParzenFilter,
            BohmanFilter => FilterType_BohmanFilter,
            BartlettFilter => FilterType_BartlettFilter,
            LagrangeFilter => FilterType_LagrangeFilter,
            LanczosFilter => FilterType_LanczosFilter,
            LanczosSharpFilter => FilterType_LanczosSharpFilter,
            Lanczos2Filter => FilterType_Lanczos2Filter,
            Lanczos2SharpFilter => FilterType_Lanczos2SharpFilter,
            RobidouxFilter => FilterType_RobidouxFilter,
            RobidouxSharpFilter => FilterType_RobidouxSharpFilter,
            CosineFilter => FilterType_CosineFilter,
            SplineFilter => FilterType_SplineFilter,
            LanczosRadiusFilter => FilterType_LanczosRadiusFilter,
            CubicSplineFilter => FilterType_CubicSplineFilter,
            SentinelFilter => FilterType_SentinelFilter,
            _ => FilterType_UndefinedFilter,
        }
    }
}
