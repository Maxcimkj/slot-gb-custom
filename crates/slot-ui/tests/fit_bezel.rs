use slot_ui::fit_game_bezel_face;

const W: usize = 720;
const H: usize = 480;
// The fitted Game Boy picture spans about x 93.3..626.7 of the 720 wide screen.
const PICTURE: std::ops::Range<usize> = 93..627;
// The screen frame's inner shadow reaches this far into the picture, as on the Actual bezel.
const SHADOW_PX: usize = 7;
const SHADOW_MAX: u8 = 135;

#[test]
fn the_fit_bezels_fill_the_side_bars_and_only_shade_the_picture_edges() {
    for colour in [false, true] {
        let rgba = fit_game_bezel_face(colour, W as u32, H as u32).expect("decode the fit bezel");
        assert_eq!(rgba.len(), W * H * 4);
        let px = |x: usize, y: usize| &rgba[(y * W + x) * 4..(y * W + x) * 4 + 4];
        for y in 0..H {
            for x in (0..PICTURE.start).chain(PICTURE.end..W) {
                assert_eq!(
                    px(x, y)[3],
                    255,
                    "colour {colour}: the bar is open at {x},{y}"
                );
            }
            for x in PICTURE {
                let p = px(x, y);
                let edge = !(PICTURE.start + SHADOW_PX..PICTURE.end - SHADOW_PX).contains(&x);
                if edge {
                    assert_eq!(
                        &p[..3],
                        [0, 0, 0],
                        "colour {colour}: the shadow at {x},{y} is not black"
                    );
                    assert!(
                        p[3] <= SHADOW_MAX,
                        "colour {colour}: the shadow at {x},{y} hides the picture"
                    );
                } else {
                    assert_eq!(
                        p[3], 0,
                        "colour {colour}: the bezel covers the picture at {x},{y}"
                    );
                }
            }
        }
        let shade = |x: usize| px(x, H / 2)[3];
        assert!(
            shade(PICTURE.start) == SHADOW_MAX && shade(PICTURE.end - 1) == SHADOW_MAX,
            "colour {colour}: no shadow on the picture's edges"
        );
    }
}
