//! Fork: the Fit picture mode for Game Boy and Game Boy Color carts.
//!
//! Fit sits between Actual and Stretch: the picture fills the screen height at the Game Boy's own
//! shape, with bars at the sides. Upstream files only call in here (see FORK-NOTES.md).

use slot_gfx::{OUT_H, OUT_W, SRC_H, SRC_W};
use slot_store::{BezelPick, Platform};

use crate::app::App;
use crate::video_mode::VideoMode;

impl VideoMode {
    /// L1: Actual → Fit → Stretch, stopping at Stretch.
    pub fn bigger(self) -> VideoMode {
        match self {
            VideoMode::Actual => VideoMode::Fit,
            VideoMode::Fit | VideoMode::Stretch => VideoMode::Stretch,
        }
    }

    /// R1: Stretch → Fit → Actual, stopping at Actual.
    pub fn smaller(self) -> VideoMode {
        match self {
            VideoMode::Stretch => VideoMode::Fit,
            VideoMode::Fit | VideoMode::Actual => VideoMode::Actual,
        }
    }
}

impl App {
    /// Only the fitted picture is softened: 3.33x would otherwise give rows of 3 and 4 lines.
    pub fn softens_picture(&self) -> bool {
        self.fits_picture()
    }
}

/// Whether the picture is fitted: Fit on any cart but a GBA one, which has no Fit.
pub fn fits(platform: Platform, mode: VideoMode) -> bool {
    mode == VideoMode::Fit && platform != Platform::Gba
}

/// The fitted picture's source rect, or None to leave it to upstream's modes. The picture's rows
/// fill the screen height; the columns follow at the same scale, centred.
pub fn source_rect(platform: Platform, mode: VideoMode) -> Option<[f32; 4]> {
    let (w, h) = platform.picture();
    if mode != VideoMode::Fit || (w, h) == (SRC_W, SRC_H) {
        return None;
    }
    let cols = h * OUT_W / OUT_H;
    let (x, y) = (SRC_W.saturating_sub(cols) / 2, (SRC_H - h) / 2);
    Some([
        x as f32 / SRC_W as f32,
        y as f32 / SRC_H as f32,
        cols as f32 / SRC_W as f32,
        h as f32 / SRC_H as f32,
    ])
}

/// The bezel over a fitted picture, or None when the picture is not fitted. The built-in bezel
/// has a Fit version; every other pick (a card bezel, AGS-102, anything upstream adds) is drawn
/// for the actual-size picture and would cover a fitted one, so it is hidden.
pub fn game_bezel(
    app: &App,
    platform: Platform,
    pick: &BezelPick,
    w: u32,
    h: u32,
) -> Option<Option<Vec<u8>>> {
    if !app.fits_picture() {
        return None;
    }
    Some(match pick {
        BezelPick::Default => slot_ui::fit_game_bezel_face(platform == Platform::Gbc, w, h),
        _ => None,
    })
}
