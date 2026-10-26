#![cfg(target_os = "macos")]
//! Fork: the Fit picture through the real game shader: full height, softened, and its grid.
//! The helpers down to `assert_even` are copies of upstream's in tests/pipeline.rs.

use slot_gfx::{Compositor, Draw, HeadlessSurface, ScreenEffect, OUT_H, OUT_W, SRC_H, SRC_W};
use std::sync::{Mutex, MutexGuard, PoisonError};

static GL: Mutex<()> = Mutex::new(());

fn compositor() -> Option<(MutexGuard<'static, ()>, HeadlessSurface, Compositor)> {
    let guard = GL.lock().unwrap_or_else(PoisonError::into_inner);
    let surface = HeadlessSurface::new().ok()?;
    let compositor = Compositor::new(&surface).ok()?;
    Some((guard, surface, compositor))
}

fn px(frame: &[u8], x: usize, y: usize) -> [u8; 3] {
    let o = (y * OUT_W as usize + x) * 4;
    [frame[o], frame[o + 1], frame[o + 2]]
}

const GB_X: usize = 40;
const GB_Y: usize = 8;
const GB_W: usize = 160;
const GB_H: usize = 144;

const GB_RECT: [f32; 4] = [
    GB_X as f32 / SRC_W as f32,
    GB_Y as f32 / SRC_H as f32,
    GB_W as f32 / SRC_W as f32,
    GB_H as f32 / SRC_H as f32,
];

fn gb_shaped(inside: impl Fn(usize, usize) -> [u8; 3], margin: [u8; 3]) -> Vec<u8> {
    let mut buf = Vec::with_capacity((SRC_W * SRC_H * 4) as usize);
    for y in 0..SRC_H as usize {
        for x in 0..SRC_W as usize {
            let in_window = (GB_X..GB_X + GB_W).contains(&x) && (GB_Y..GB_Y + GB_H).contains(&y);
            let rgb = match in_window {
                true => inside(x - GB_X, y - GB_Y),
                false => margin,
            };
            buf.extend_from_slice(&[rgb[2], rgb[1], rgb[0], 0]);
        }
    }
    buf
}

fn cell_means(profile: &[f32], cells: usize) -> Vec<f32> {
    let span = profile.len() as f32 / cells as f32;
    (0..cells)
        .map(|k| {
            let (a, b) = (k as f32 * span, (k + 1) as f32 * span);
            let lit: f32 = (a.floor() as usize..(b.ceil() as usize).min(profile.len()))
                .map(|x| (b.min(x as f32 + 1.0) - a.max(x as f32)).max(0.0) * profile[x])
                .sum();
            lit / span
        })
        .collect()
}

fn assert_even(profile: &[f32], cells: usize, spread: f32, what: &str) {
    let means = cell_means(profile, cells);
    let (lo, hi) = means
        .iter()
        .fold((f32::MAX, f32::MIN), |(lo, hi), m| (lo.min(*m), hi.max(*m)));
    assert!(hi < 0.97, "{what}: a cell has no grid line, mean {hi}");
    assert!(
        hi - lo < spread,
        "{what}: cells run from {lo} to {hi}, unevenly lined"
    );
}

const FIT_RECT: [f32; 4] = [
    12.0 / SRC_W as f32,
    GB_Y as f32 / SRC_H as f32,
    216.0 / SRC_W as f32,
    GB_H as f32 / SRC_H as f32,
];

fn fitted(c: &mut Compositor, soft: bool, inside: impl Fn(usize, usize) -> [u8; 3]) -> Vec<u8> {
    c.set_picture([
        GB_RECT[0],
        GB_RECT[1],
        GB_RECT[0] + GB_RECT[2],
        GB_RECT[1] + GB_RECT[3],
    ]);
    c.set_game_source_rect(FIT_RECT);
    c.set_soft_picture(soft);
    c.set_screen_effect(ScreenEffect::None);
    c.set_screen_power(1.0);
    c.begin_frame();
    c.upload_game(&gb_shaped(inside, [0, 0, 0]));
    c.draw_game();
    c.read_frame()
}

fn fitted_stripes(c: &mut Compositor, soft: bool) -> Vec<u8> {
    fitted(c, soft, |_, y| match y % 4 == 0 {
        true => [0, 0, 0],
        false => [255, 255, 255],
    })
}

/// How dark each line of the stripe pattern is in total, one entry per stripe.
fn stripe_weights(line: impl Iterator<Item = [u8; 3]>) -> Vec<f32> {
    let mut weights = Vec::new();
    let mut run = 0.0;
    for rgb in line {
        let dark = 1.0 - rgb[1] as f32 / 255.0;
        if dark > 0.01 {
            run += dark;
        } else if run > 0.0 {
            weights.push(run);
            run = 0.0;
        }
    }
    // The first stripe can start above the screen; only whole ones are compared.
    weights
        .split_first()
        .map(|(_, rest)| rest.to_vec())
        .unwrap_or_default()
}

fn spread(weights: &[f32]) -> f32 {
    let max = weights.iter().cloned().fold(f32::MIN, f32::max);
    let min = weights.iter().cloned().fold(f32::MAX, f32::min);
    max - min
}

#[test]
fn the_fitted_picture_fills_the_height_with_bars_only_at_the_sides() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    let frame = fitted(&mut c, true, |_, _| [255, 255, 255]);
    let lit = |x: usize, y: usize| px(&frame, x, y).iter().any(|v| *v > 8);
    let (w, h) = (OUT_W as usize, OUT_H as usize);
    assert!(
        lit(w / 2, 0) && lit(w / 2, h - 1),
        "the picture does not reach the top and bottom"
    );
    assert!(
        !lit(91, h / 2) && !lit(w - 92, h / 2),
        "the side bars are not black"
    );
    assert!(
        lit(95, h / 2) && lit(w - 96, h / 2),
        "the picture is narrower than 534"
    );
}

fn fitted_grid(c: &mut Compositor, soft: bool) -> Vec<u8> {
    c.set_picture([
        GB_RECT[0],
        GB_RECT[1],
        GB_RECT[0] + GB_RECT[2],
        GB_RECT[1] + GB_RECT[3],
    ]);
    c.set_game_source_rect(FIT_RECT);
    c.fit((OUT_W, OUT_H));
    c.set_soft_picture(soft);
    c.set_screen_effect(ScreenEffect::Grid);
    c.set_screen_power(1.0);
    c.begin_frame();
    c.upload_game(&gb_shaped(|_, _| [0xc0, 0xc0, 0xc0], [0, 0, 0]));
    c.draw_game();
    c.read_frame()
}

/// The fitted picture's brightness down its brightest column and across its brightest row, each
/// relative to its brightest pixel, so neither runs along a grid line. Across covers whole game
/// pixels only: x 100..620 is 156 of them.
fn fitted_grid_profiles(frame: &[u8]) -> (Vec<f32>, Vec<f32>) {
    let g = |x: usize, y: usize| px(frame, x, y)[1] as f32;
    let norm = |v: Vec<f32>| {
        let lit = v.iter().cloned().fold(0.0, f32::max);
        v.into_iter().map(|g| g / lit).collect::<Vec<_>>()
    };
    let brightest = |lines: std::ops::Range<usize>, sum: &dyn Fn(usize) -> f32| {
        lines.max_by(|a, b| sum(*a).total_cmp(&sum(*b))).unwrap()
    };
    let (h, xs) = (OUT_H as usize, 100..620);
    let x = brightest(xs.clone(), &|x| (0..h).map(|y| g(x, y)).sum());
    let y = brightest(0..h, &|y| xs.clone().map(|x| g(x, y)).sum());
    let down = norm((0..h).map(|y| g(x, y)).collect());
    let across = norm(xs.map(|x| g(x, y)).collect());
    (down, across)
}

#[test]
fn the_fitted_grid_lines_every_game_pixel_as_in_the_other_modes() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    // Softening leaves a flat picture as it is, so any difference would be the grid itself. Only
    // the picture's outer columns, x 93 and 626, blend into the bars. A GPU may round the softened
    // sample a step differently, so 1 per channel is allowed.
    let soft = fitted_grid(&mut c, true);
    let hard = fitted_grid(&mut c, false);
    let differs = (0..OUT_H as usize)
        .flat_map(|y| (94..626).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            let (a, b) = (px(&soft, x, y), px(&hard, x, y));
            (0..3).any(|ch| a[ch].abs_diff(b[ch]) > 1)
        })
        .count();
    assert_eq!(
        differs, 0,
        "the fitted picture's grid differs from upstream's grid"
    );
    // Every game pixel has a line: averaged per game pixel, the cells darken alike. This does not
    // see a line's shape; at 3.33x some are one dark pixel and others two lighter ones, as in
    // Stretch's rows.
    let (down, across) = fitted_grid_profiles(&soft);
    assert_even(&down, GB_H, 0.15, "fitted rows");
    assert_even(&across, 156, 0.15, "fitted columns");
    // Only one line per game pixel, on its edge: half the half-pixel cells have none.
    assert!(
        spread(&cell_means(&down, 2 * GB_H)) > 0.3,
        "the fitted grid has lines inside the game pixels"
    );
}

#[test]
fn softening_gives_every_fitted_game_boy_row_the_same_weight() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    let x = OUT_W as usize / 2 + 6;
    let rows = |frame: &[u8]| stripe_weights((0..OUT_H as usize).map(|y| px(frame, x, y)));

    let hard = rows(&fitted_stripes(&mut c, false));
    assert!(
        spread(&hard) >= 0.9,
        "hard edges were already even: {hard:?}"
    );

    let soft = rows(&fitted_stripes(&mut c, true));
    assert_eq!(soft.len(), hard.len(), "softening lost a stripe: {soft:?}");
    assert!(spread(&soft) < 0.4, "softened rows are uneven: {soft:?}");
}

#[test]
fn a_screenshot_is_never_softened_even_after_a_fitted_game() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    let shot: Vec<u8> = (0..(SRC_W * SRC_H) as usize)
        .flat_map(|i| match (i % SRC_W as usize + i / SRC_W as usize) % 2 {
            0 => [250, 250, 250, 255],
            _ => [10, 10, 10, 255],
        })
        .collect();
    let tex = c.create_texture_nearest(SRC_W, SRC_H, &shot);
    c.set_screen_effect(ScreenEffect::Grid);
    c.set_screen_power(1.0);
    let still = |c: &mut Compositor, soft: bool| {
        c.set_soft_picture(soft);
        c.begin_frame();
        c.draw_list(&[Draw::Shot { tex }]);
        c.read_frame()
    };
    let plain = still(&mut c, false);
    assert!(
        still(&mut c, true) == plain,
        "the screenshot changed because the last game was fitted"
    );
}
