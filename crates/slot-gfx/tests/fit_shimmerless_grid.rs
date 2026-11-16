#![cfg(target_os = "macos")]
//! Fork: the Shimmerless Grid filter through the real game shader, checked against fixed
//! expectations computed from the game texture and the layout alone. No other filter is rendered,
//! so upstream changes to Shimmerless or Grid cannot change what these tests expect.
//! `compositor`, `gb_shaped`, `cell_means` and `assert_even` are copies of upstream's helpers in
//! tests/pipeline.rs.

use slot_gfx::{game_layout, Compositor, HeadlessSurface, ScreenEffect, Sizing, SRC_H, SRC_W};
use std::sync::{Mutex, MutexGuard, PoisonError};

static GL: Mutex<()> = Mutex::new(());

fn compositor() -> Option<(MutexGuard<'static, ()>, HeadlessSurface, Compositor)> {
    let guard = GL.lock().unwrap_or_else(PoisonError::into_inner);
    let surface = HeadlessSurface::new().ok()?;
    let compositor = Compositor::new(&surface).ok()?;
    Some((guard, surface, compositor))
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

/// The RG SP's panel, where Fit is about 3.33x, and a 4:3 one.
const PANELS: [(u32, u32); 2] = [(720, 480), (640, 480)];

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

struct Frame {
    rgba: Vec<u8>,
    w: usize,
    h: usize,
}

impl Frame {
    fn px(&self, x: usize, y: usize) -> [u8; 3] {
        let o = (y * self.w + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2]]
    }
}

/// A Game Boy picture laid out as upstream lays it out for `sizing` on `panel`.
fn shown(
    c: &mut Compositor,
    panel: (u32, u32),
    sizing: Sizing,
    effect: ScreenEffect,
    inside: &dyn Fn(usize, usize) -> [u8; 3],
) -> Frame {
    c.set_game_layout(game_layout(panel, (GB_W as u32, GB_H as u32), sizing));
    c.fit(panel);
    c.set_picture([
        GB_RECT[0],
        GB_RECT[1],
        GB_RECT[0] + GB_RECT[2],
        GB_RECT[1] + GB_RECT[3],
    ]);
    c.set_game_source_rect(GB_RECT);
    c.set_screen_effect(effect);
    c.set_screen_power(1.0);
    c.begin_frame();
    c.upload_game(&gb_shaped(inside, [0, 0, 0]));
    c.draw_game();
    let (w, h) = c.frame_size();
    Frame {
        rgba: c.read_frame(),
        w: w as usize,
        h: h as usize,
    }
}

fn flat(_: usize, _: usize) -> [u8; 3] {
    [0xc0, 0xc0, 0xc0]
}

fn white(_: usize, _: usize) -> [u8; 3] {
    [255, 255, 255]
}

/// Colours that change on every game pixel, in every channel.
fn colours(x: usize, y: usize) -> [u8; 3] {
    let v = ((x * 37 + y * 91) % 7) as u8 * 36;
    [v, 255 - v, (x % 3) as u8 * 120]
}

fn stripes(_: usize, y: usize) -> [u8; 3] {
    match y.is_multiple_of(4) {
        true => [0, 0, 0],
        false => [255, 255, 255],
    }
}

/// Where the fitted picture sits: upstream centres 533 columns.
fn picture_x(panel: (u32, u32)) -> std::ops::Range<usize> {
    let left = (panel.0 as usize - 533) / 2;
    left..left + 533
}

/// How much the grid darkens each screen pixel: the filter over a white picture.
fn grid_factor(c: &mut Compositor, panel: (u32, u32), sizing: Sizing) -> Frame {
    shown(c, panel, sizing, ScreenEffect::ShimmerlessGrid, &white)
}

/// The game pixel (as fractional texel coordinates) under the centre of screen pixel `(x, y)`.
fn game_at(rect: [f32; 4], x: usize, y: usize) -> (f32, f32) {
    let [rx, ry, rw, rh] = rect;
    (
        (x as f32 + 0.5 - rx) * GB_W as f32 / rw,
        (y as f32 + 0.5 - ry) * GB_H as f32 / rh,
    )
}

/// A screen pixel's colour with the grid's darkening taken out, or `None` on a line too dark to
/// undo reliably.
fn under_grid(f: &Frame, lines: &Frame, x: usize, y: usize) -> Option<[f32; 3]> {
    let k = lines.px(x, y)[1] as f32 / 255.0;
    let p = f.px(x, y);
    (k >= 0.4).then(|| p.map(|v| v as f32 / k))
}

#[test]
fn the_fitted_picture_fills_the_height_with_bars_only_at_the_sides() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    let f = shown(
        &mut c,
        (720, 480),
        Sizing::Fit,
        ScreenEffect::ShimmerlessGrid,
        &white,
    );
    let lit = |x: usize, y: usize| f.px(x, y).iter().any(|v| *v > 8);
    let (w, h) = (f.w, f.h);
    assert!(
        lit(w / 2 + 1, 1) && lit(w / 2 + 1, h - 2),
        "the picture does not reach the top and bottom"
    );
    assert!(
        !lit(91, h / 2) && !lit(w - 92, h / 2),
        "the side bars are not black"
    );
    assert!(
        lit(95, h / 2 + 1) && lit(w - 96, h / 2 + 1),
        "the picture is narrower than 533"
    );
}

#[test]
fn the_fitted_shimmerless_grid_lines_every_game_pixel_evenly() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    for panel in PANELS {
        let f = shown(
            &mut c,
            panel,
            Sizing::Fit,
            ScreenEffect::ShimmerlessGrid,
            &flat,
        );
        let g = |x: usize, y: usize| f.px(x, y)[1] as f32 / 0xc0 as f32;
        // Down the brightest column and across the brightest row, so neither runs along a line.
        let xs = picture_x(panel);
        let x = xs
            .clone()
            .max_by(|a, b| {
                let sum = |x: usize| (0..f.h).map(|y| g(x, y)).sum::<f32>();
                sum(*a).total_cmp(&sum(*b))
            })
            .unwrap();
        let y = (0..f.h)
            .max_by(|a, b| {
                let sum = |y: usize| xs.clone().map(|x| g(x, y)).sum::<f32>();
                sum(*a).total_cmp(&sum(*b))
            })
            .unwrap();
        let down: Vec<f32> = (0..f.h).map(|y| g(x, y)).collect();
        let across: Vec<f32> = xs.map(|x| g(x, y)).collect();
        // Every game pixel has a line: averaged per game pixel, the cells darken alike.
        assert_even(&down, GB_H, 0.18, "Shimmerless Grid rows");
        assert_even(&across, GB_W, 0.18, "Shimmerless Grid columns");
        // Only one line per game pixel, on its edge: half the half-pixel cells have none.
        let halves = cell_means(&down, 2 * GB_H);
        let spread = halves.iter().cloned().fold(f32::MIN, f32::max)
            - halves.iter().cloned().fold(f32::MAX, f32::min);
        assert!(
            spread > 0.3,
            "{panel:?}: the grid has lines inside the game pixels"
        );
    }
}

#[test]
fn shimmerless_grid_gives_every_fitted_row_the_same_weight() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    // Every fourth game row is black. Down one column, with the grid's darkening taken out, each
    // black row must weigh the same: 480 / 144 = 3.33 screen lines.
    let lines = grid_factor(&mut c, (720, 480), Sizing::Fit);
    let f = shown(
        &mut c,
        (720, 480),
        Sizing::Fit,
        ScreenEffect::ShimmerlessGrid,
        &stripes,
    );
    let x = 360 + 6;
    let mut weights = Vec::new();
    let mut run = 0.0;
    for y in 0..f.h {
        let lit = f.px(x, y)[1] as f32 / (lines.px(x, y)[1] as f32).max(1.0);
        let dark = 1.0 - lit.min(1.0);
        if dark > 0.02 {
            run += dark;
        } else if run > 0.0 {
            weights.push(run);
            run = 0.0;
        }
    }
    // The first stripe starts at the top edge; only whole ones are compared.
    let weights = &weights[1..];
    assert_eq!(weights.len(), GB_H / 4 - 1, "stripes: {weights:?}");
    let want = 480.0 / GB_H as f32;
    assert!(
        weights.iter().all(|w| (w - want).abs() < 0.2),
        "every black row should weigh {want} lines: {weights:?}"
    );
}

#[test]
fn shimmerless_grid_in_fit_blends_only_neighbouring_game_pixels_and_stays_sharp() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    let panel = (720, 480);
    let rect = game_layout(panel, (GB_W as u32, GB_H as u32), Sizing::Fit).rect;
    let lines = grid_factor(&mut c, panel, Sizing::Fit);
    let f = shown(
        &mut c,
        panel,
        Sizing::Fit,
        ScreenEffect::ShimmerlessGrid,
        &colours,
    );
    let texel = |gx: i32, gy: i32| {
        colours(
            gx.clamp(0, GB_W as i32 - 1) as usize,
            gy.clamp(0, GB_H as i32 - 1) as usize,
        )
    };
    let (mut checked, mut pure) = (0, 0);
    for y in 1..f.h - 1 {
        for x in picture_x(panel).start + 1..picture_x(panel).end - 1 {
            let Some(got) = under_grid(&f, &lines, x, y) else {
                continue;
            };
            let (gx, gy) = game_at(rect, x, y);
            let (x0, y0) = ((gx - 0.5).floor() as i32, (gy - 0.5).floor() as i32);
            let near = [
                texel(x0, y0),
                texel(x0 + 1, y0),
                texel(x0, y0 + 1),
                texel(x0 + 1, y0 + 1),
            ];
            for ch in 0..3 {
                let lo = near.iter().map(|t| t[ch]).min().unwrap() as f32;
                let hi = near.iter().map(|t| t[ch]).max().unwrap() as f32;
                assert!(
                    got[ch] >= lo - 4.0 && got[ch] <= hi + 4.0,
                    "({x}, {y}) channel {ch}: {} is not a blend of the game pixels it touches \
                     ({lo}..{hi})",
                    got[ch]
                );
            }
            let own = texel(gx.floor() as i32, gy.floor() as i32);
            if (0..3).all(|ch| (got[ch] - own[ch] as f32).abs() <= 4.0) {
                pure += 1;
            }
            checked += 1;
        }
    }
    assert!(checked > 100_000, "only {checked} pixels were checked");
    // Sharp: only a screen line at each game pixel edge is blended, so most pixels are one
    // game pixel's own colour. A soft filter would blend nearly all of them.
    assert!(
        pure * 10 > checked * 4,
        "only {pure} of {checked} pixels are a single game pixel's colour: the picture is blurred"
    );
}

#[test]
fn shimmerless_grid_at_a_whole_scale_does_not_blend() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    // Integer on the RG SP is exactly 3x: there is nothing to smooth, so every screen pixel is its
    // own game pixel's colour, only darkened by the grid.
    let panel = (720, 480);
    let rect = game_layout(panel, (GB_W as u32, GB_H as u32), Sizing::Integer).rect;
    assert_eq!(rect[2], 3.0 * GB_W as f32, "Integer is no longer 3x here");
    let lines = grid_factor(&mut c, panel, Sizing::Integer);
    let f = shown(
        &mut c,
        panel,
        Sizing::Integer,
        ScreenEffect::ShimmerlessGrid,
        &colours,
    );
    let [rx, ry, rw, rh] = rect.map(|v| v as usize);
    // Pixels under a dark line are skipped (see `under_grid`): at 3x that is 5 of every 9.
    let mut checked = 0;
    for y in ry..ry + rh {
        for x in rx..rx + rw {
            let Some(got) = under_grid(&f, &lines, x, y) else {
                continue;
            };
            let own = colours((x - rx) / 3, (y - ry) / 3);
            assert!(
                (0..3).all(|ch| (got[ch] - own[ch] as f32).abs() <= 4.0),
                "({x}, {y}): {got:?} is not its game pixel's {own:?}"
            );
            checked += 1;
        }
    }
    assert!(checked > 80_000, "only {checked} pixels were checked");
}
