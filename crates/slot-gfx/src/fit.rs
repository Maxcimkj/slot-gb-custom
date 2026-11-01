//! Fork: the Fit picture's softening, kept out of upstream's shader.
//!
//! `game_frag` patches upstream's `GAME_FRAG` at a few anchors instead of editing it, so an
//! upstream change to the shader merges cleanly. If upstream rewrites an anchor, slot runs
//! upstream's shader unpatched (Fit then only loses its softening), and
//! `the_fit_patches_apply_to_upstreams_game_shader` below fails on any machine, GPU or not, so
//! the sync catches it.

/// Sharp bilinear: hard texels that blend only across the one screen line where two meet.
const SOFT_TEXEL: &str = "uniform float u_soft;
varying vec2 v_uv;
vec3 soft_texel(vec2 uv) {
    vec2 scale = u_rect.zw * u_fbo / (u_uv.zw * u_src);
    vec2 t = uv * u_src - 0.5;
    vec2 i = floor(t);
    vec2 s = clamp((t - i - 0.5) * scale + 0.5, 0.0, 1.0);
    vec2 a = (i + 0.5) / u_src;
    vec2 b = (i + 1.5) / u_src;
    vec3 top = mix(texture2D(u_game, a).rgb, texture2D(u_game, vec2(b.x, a.y)).rgb, s.x);
    vec3 bottom = mix(texture2D(u_game, vec2(a.x, b.y)).rgb, texture2D(u_game, b).rgb, s.x);
    return mix(top, bottom, s.y);
}
";

/// (what it is, upstream's text, the fork's text). Each anchor must occur exactly once.
const PATCHES: &[(&str, &str, &str)] = &[
    ("the softening function", "varying vec2 v_uv;\n", SOFT_TEXEL),
    (
        "the first sample",
        "vec3 c = texture2D(u_game, uv).rgb;",
        // A branch on a uniform: every pixel of a frame takes the same side, so Actual, Stretch
        // and GBA skip soft_texel's four fetches. A ternary may be flattened to run both.
        "vec3 c;
    if (u_soft > 0.5) {
        c = soft_texel(uv);
    } else {
        c = texture2D(u_game, uv).rgb;
    }",
    ),
];

/// Upstream's game shader with the Fit softening patched in, or upstream's shader as it
/// is when an anchor is missing or not unique.
pub(crate) fn game_frag(upstream: &str) -> String {
    let mut src = upstream.to_string();
    for (what, from, to) in PATCHES {
        if src.matches(from).count() != 1 {
            eprintln!(
                "slot: fit: upstream's game shader changed around {what}; Fit is not softened"
            );
            return upstream.to_string();
        }
        src = src.replacen(from, to, 1);
    }
    src
}

/// The `u_soft` uniform: on while the picture is fitted.
pub(crate) struct Soft {
    at: gl::types::GLint,
    on: bool,
}

impl Soft {
    pub(crate) fn new(prog: gl::types::GLuint) -> Soft {
        Soft {
            at: crate::gl::uniform_location(prog, "u_soft"),
            on: false,
        }
    }

    pub(crate) fn set(&mut self, on: bool) {
        self.on = on;
    }

    /// Sends the flag; the game program must be in use. Only the live game is ever softened: a
    /// still (a screenshot) is drawn as upstream draws it, whatever the last game's mode was.
    pub(crate) fn upload(&self, live_game: bool) {
        let on = self.on && live_game;
        unsafe { gl::Uniform1f(self.at, if on { 1.0 } else { 0.0 }) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fit_patches_apply_to_upstreams_game_shader() {
        let src = game_frag(crate::shaders::GAME_FRAG);
        for (_, _, to) in PATCHES {
            assert!(src.contains(to));
        }
    }

    #[test]
    fn the_other_modes_skip_the_softening_with_a_branch() {
        let src = game_frag(crate::shaders::GAME_FRAG);
        assert!(src.contains("if (u_soft > 0.5) {"), "no branch on u_soft");
        assert!(
            !src.contains("? soft_texel"),
            "a ternary may run soft_texel's four fetches in every mode"
        );
    }

    #[test]
    fn a_shader_the_patches_do_not_fit_is_left_as_upstream_wrote_it() {
        let changed = crate::shaders::GAME_FRAG.replace("vec3 c = texture2D", "vec3 c = texture");
        assert_eq!(game_frag(&changed), changed);
    }
}
