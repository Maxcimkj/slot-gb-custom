//! Fork: the Fit picture's softening and its finer grid, kept out of upstream's shader.
//!
//! `game_frag` patches upstream's `GAME_FRAG` at a few anchors instead of editing it, so an
//! upstream change to the shader merges cleanly. If upstream rewrites an anchor, slot runs
//! upstream's shader unpatched (Fit then only loses its softening and finer grid), and the unit
//! test below fails on any machine, GPU or not, so the sync catches it.

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
const PATCHES: [(&str, &str, &str); 3] = [
    ("the softening function", "varying vec2 v_uv;\n", SOFT_TEXEL),
    (
        "the first sample",
        "vec3 c = texture2D(u_game, uv).rgb;",
        "vec3 c = u_soft > 0.5 ? soft_texel(uv) : texture2D(u_game, uv).rgb;",
    ),
    // The softened (fitted) picture gets two grid cells per game pixel; the rest keep one.
    (
        "the grid",
        "vec2 hp = 0.5 * u_uv.zw * u_src / (u_rect.zw * u_fbo);
        vec2 edge = (grid_band(f + hp) - grid_band(f - hp)) / (2.0 * hp);",
        "float cells = u_soft > 0.5 ? 2.0 : 1.0;
        vec2 hp = 0.5 * cells * u_uv.zw * u_src / (u_rect.zw * u_fbo);
        vec2 g = fract(uv * u_src * cells);
        vec2 edge = (grid_band(g + hp) - grid_band(g - hp)) / (2.0 * hp);",
    ),
];

/// Upstream's game shader with the Fit softening and grid patched in, or upstream's shader as it
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
    fn a_shader_the_patches_do_not_fit_is_left_as_upstream_wrote_it() {
        let changed = crate::shaders::GAME_FRAG.replace("vec3 c = texture2D", "vec3 c = texture");
        assert_eq!(game_frag(&changed), changed);
    }
}
