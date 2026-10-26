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
    /// Whether the game on screen is fitted. `picture_mode` is the platform and mode the core was
    /// opened with, which outlive the cart: a cart must be seated, and be of that platform, so
    /// the shelf after a fitted game, or a GBA cart going in after one, never counts as fitted.
    pub fn fits_picture(&self) -> bool {
        let (platform, mode) = self.picture_mode();
        fits(platform, mode) && self.seated_cart().map(|c| c.platform) == Some(platform)
    }

    /// Only the fitted picture is softened: 3.33x would otherwise give rows of 3 and 4 lines. Its
    /// own decision, though today it is the same as `fits_picture`.
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
    if !fits(platform, mode) {
        return None;
    }
    let (_, h) = platform.picture();
    let cols = h * OUT_W / OUT_H;
    let (x, y) = (SRC_W.saturating_sub(cols) / 2, (SRC_H - h) / 2);
    Some([
        x as f32 / SRC_W as f32,
        y as f32 / SRC_H as f32,
        cols as f32 / SRC_W as f32,
        h as f32 / SRC_H as f32,
    ])
}

/// The frontend's bezel cache key: platform, pick, size and whether the picture is fitted.
pub type BezelKey = (Platform, BezelPick, (u32, u32), bool);

/// The bezel cache key for what is on screen. Stretch shows no bezel, so while stretched the
/// cached bezel is kept as it is: leaving Fit for Stretch, or coming back, loads nothing.
pub fn bezel_key(
    app: &App,
    (platform, pick, size): (Platform, BezelPick, (u32, u32)),
    cached: Option<&BezelKey>,
) -> BezelKey {
    let fits = match cached {
        Some((p, b, s, fits))
            if app.picture_mode().1 == VideoMode::Stretch
                && (*p, b, *s) == (platform, &pick, size) =>
        {
            *fits
        }
        _ => app.fits_picture(),
    };
    (platform, pick, size, fits)
}

/// The bezel over a fitted picture. The built-in bezel has a Fit version; every other pick (a
/// card bezel, AGS-102, anything upstream adds) is drawn for the actual-size picture and would
/// cover a fitted one, so it is hidden, and never read from the card.
pub fn game_bezel(platform: Platform, pick: &BezelPick, w: u32, h: u32) -> Option<Vec<u8>> {
    match pick {
        BezelPick::Default => slot_ui::fit_game_bezel_face(platform == Platform::Gbc, w, h),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_crop_and_the_fitted_check_agree() {
        for platform in [Platform::Gb, Platform::Gbc, Platform::Gba] {
            for mode in [VideoMode::Actual, VideoMode::Fit, VideoMode::Stretch] {
                assert_eq!(
                    source_rect(platform, mode).is_some(),
                    fits(platform, mode),
                    "{platform:?} in {mode:?}"
                );
            }
        }
    }
}
