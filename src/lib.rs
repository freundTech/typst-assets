//! Assets for the Typst compiler.
//!
//! These are not part of the main compiler crate to keep its size down.

macro_rules! asset {
    ($path:literal) => {
        include_bytes!(concat!("../files/", $path)).as_slice()
    };
}

macro_rules! add {
    ($ident:ident, $path:literal) => {
        pub const $ident: &[u8] = asset!($path);
    };
}

/// ICU data.
pub mod icu {
    // Generated by the following command:
    //
    // ```sh
    // icu4x-datagen --locales full \
    //               --format blob \
    //               --keys-for-bin target/debug/typst \
    //               --out ../typst-assets/files/icu/icu.postcard \
    //               --overwrite
    // ```
    //
    // Install icu_datagen with `cargo install icu_datagen`.
    add!(ICU, "icu/icu.postcard");

    // Generated by the following command:
    //
    // ```sh
    // icu4x-datagen --locales zh ja \
    //               --format blob \
    //               --keys segmenter/line@1 \
    //               --out ../typst-assets/files/icu/icu_cj_segment.postcard \
    //               --overwrite
    // ```
    //
    // The used icu_datagen should be patched by
    // https://github.com/peng1999/icu4x/commit/b9beb6cbf633d61fc3d7983e5baf7f4449fbfae5
    add!(ICU_CJ_SEGMENT, "icu/icu_cj_segment.postcard");
}

/// ICC profiles.
pub mod icc {
    // The ICC profile used to convert from CMYK to RGB.
    //
    // This is a minimal CMYK profile that only contains the necessary
    // information to convert from CMYK to RGB. It is based on the CGATS TR
    // 001-1995 specification. See
    // https://github.com/saucecontrol/Compact-ICC-Profiles#cmyk.
    add!(CMYK_TO_XYZ, "icc/CMYK-to-XYZ.icc");

    add!(S_GREY_V4, "icc/sGrey-v4.icc");
    add!(S_RGB_V4, "icc/sRGB-v4.icc");
}

/// Bundled fonts.
pub fn fonts() -> impl Iterator<Item = &'static [u8]> {
    [
        asset!("fonts/LinLibertine_R.ttf"),
        asset!("fonts/LinLibertine_RB.ttf"),
        asset!("fonts/LinLibertine_RBI.ttf"),
        asset!("fonts/LinLibertine_RI.ttf"),
        asset!("fonts/NewCMMath-Book.otf"),
        asset!("fonts/NewCMMath-Regular.otf"),
        asset!("fonts/NewCM10-Regular.otf"),
        asset!("fonts/NewCM10-Bold.otf"),
        asset!("fonts/NewCM10-Italic.otf"),
        asset!("fonts/NewCM10-BoldItalic.otf"),
        asset!("fonts/DejaVuSansMono-Bold.ttf"),
        asset!("fonts/DejaVuSansMono-BoldOblique.ttf"),
        asset!("fonts/DejaVuSansMono-Oblique.ttf"),
        asset!("fonts/DejaVuSansMono.ttf"),
    ]
    .into_iter()
}