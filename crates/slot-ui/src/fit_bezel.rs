//! Fork: the built-in bezels for the Fit picture, kept out of upstream's backdrop module.

use crate::art;

const BEZEL_GB_FIT_PNG: &[u8] = include_bytes!("../../../assets/bezel_gb_fit.png");
const BEZEL_GBC_FIT_PNG: &[u8] = include_bytes!("../../../assets/bezel_gbc_fit.png");

/// The built-in bezel for the full-height picture: the Actual bezel's screen surround, cut down to
/// the side bars, with its inner shadow over the picture's left and right edges.
pub fn fit_game_bezel_face(colour: bool, w: u32, h: u32) -> Option<Vec<u8>> {
    let png = if colour {
        BEZEL_GBC_FIT_PNG
    } else {
        BEZEL_GB_FIT_PNG
    };
    art::cover_bytes(png, w, h)
}
