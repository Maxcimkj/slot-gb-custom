//! Fork: Shimmerless Grid is its own branch in upstream's game shader, with its own copy of the
//! Shimmerless and Grid code, so upstream changes to those modes do not change it. This checks it
//! is still there, first in the chain, matching exactly `ScreenEffect::ShimmerlessGrid`'s number.
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
