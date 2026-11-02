//! Fork: Shimmerless Grid is its own branch in upstream's game shader, with its own copy of the
//! Shimmerless and Grid code, so upstream changes to those modes do not change it. This checks it
//! is still there, first in the chain, matching exactly `ScreenEffect::ShimmerlessGrid`'s number,
//! and that it still smooths and then draws the grid. The rendering itself is checked by
//! tests/fit_shimmerless_grid.rs, which needs macOS; these run on the Linux sync too.
//! Runs on any machine: it reads the shader source as text, ignoring whitespace, so upstream
//! re-indenting the shader does not fail it.

use slot_gfx::ScreenEffect;

const SHADERS: &str = include_str!("../src/shaders.rs");

fn without_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn shimmerless_grid_is_the_first_branch_and_matches_only_its_effect() {
    let mode = ScreenEffect::ShimmerlessGrid.mode();
    let branch = format!("if(!inside){{}}elseif(abs(u_mode-{mode:.1})<0.5){{");
    assert!(
        without_whitespace(SHADERS).contains(&branch),
        "the game shader's first branch is not Shimmerless Grid matching exactly mode {mode} \
         (ScreenEffect::ShimmerlessGrid): want `}} else if (abs(u_mode - {mode:.1}) < 0.5) {{` right \
         after `if (!inside) {{ }}`"
    );
}

/// The body of the Shimmerless Grid branch, without whitespace.
fn branch_body() -> String {
    let src = without_whitespace(SHADERS);
    let open = format!(
        "elseif(abs(u_mode-{:.1})<0.5){{",
        ScreenEffect::ShimmerlessGrid.mode()
    );
    let start = src.find(&open).expect("no Shimmerless Grid branch") + open.len();
    let len = src[start..]
        .find("}elseif(")
        .expect("no branch after Shimmerless Grid");
    src[start..start + len].to_string()
}

#[test]
fn shimmerless_grid_smooths_the_sample_and_then_darkens_the_pixel_edges() {
    let body = branch_body();
    let smooth = without_whitespace("c = mix(top, bottom, t.y);");
    let grid = without_whitespace("c *= 1.0 - 0.75 * max(edge.x, edge.y);");
    let (Some(s), Some(g)) = (body.find(&smooth), body.find(&grid)) else {
        panic!("the Shimmerless Grid branch lost its smoothing or its grid lines: {body}");
    };
    assert!(s < g, "the grid is drawn before the smoothing: {body}");
    assert!(
        body.contains("clamp(b-0.5,lo,hi)") && body.contains("grid_band(f+hp)"),
        "the Shimmerless Grid branch no longer samples like Shimmerless or lines like Grid: {body}"
    );
}
