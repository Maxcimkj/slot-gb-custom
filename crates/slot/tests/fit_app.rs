//! Fork: the Fit picture mode as the app sees it. Upstream's two tests of L1/R1 and of the
//! remembered mode, which expect Actual and Stretch only, live here in their Fit versions.

mod common;

use std::path::Path;
use std::time::{Duration, Instant};

use slot::app::Phase;
use slot::session::Session;
use slot::video_mode::{video_mode_for, VideoMode, VIDEO_MODE_FILE};
use slot_gfx::WHOLE_TEXTURE;
use slot_input::{Btn, RawEvent};
use slot_retro::ButtonMask;
use slot_ui::TexId;

fn session_playing(root: &Path) -> Session {
    common::clocked(root);
    let mut s = Session::boot(root.to_path_buf());
    s.feed([RawEvent::Down(Btn::A)], 16);
    s.feed([RawEvent::Up(Btn::A)], 32);
    let deadline = Instant::now() + Duration::from_secs(10);
    while !matches!(s.app().phase(), Phase::Playing { .. }) {
        assert!(Instant::now() < deadline, "the cart never seated");
        s.update(1.0 / 60.0);
        std::thread::sleep(Duration::from_millis(1));
    }
    s
}

const GB_WINDOW: [f32; 4] = [40.0 / 240.0, 0.0, 160.0 / 240.0, 1.0];
const FIT_WINDOW: [f32; 4] = [12.0 / 240.0, 8.0 / 160.0, 216.0 / 240.0, 144.0 / 160.0];

fn pad(s: &Session) -> u16 {
    s.emu().expect("no core in the slot").input().0
}

#[test]
fn l_and_r_change_the_mode_on_a_game_boy_cart_and_not_on_a_gba_one() {
    let d = common::tmp_root_with_gb_carts(&["Tetris", "Zzz"]);
    let mut s = session_playing(d.path());
    assert_eq!(
        s.app().source_rect(),
        WHOLE_TEXTURE,
        "it did not open actual size"
    );

    s.feed([RawEvent::Down(Btn::L1)], 100);
    assert_eq!(
        s.app().source_rect(),
        FIT_WINDOW,
        "L did not fit the picture to the height"
    );
    assert_eq!(pad(&s) & ButtonMask::L, 0, "L reached the game as well");
    s.feed([RawEvent::Up(Btn::L1)], 120);

    s.feed([RawEvent::Down(Btn::L1)], 140);
    s.feed([RawEvent::Up(Btn::L1)], 160);
    assert_eq!(
        s.app().source_rect(),
        GB_WINDOW,
        "a second L did not stretch the picture"
    );
    s.feed([RawEvent::Down(Btn::L1)], 180);
    s.feed([RawEvent::Up(Btn::L1)], 190);
    assert_eq!(s.app().source_rect(), GB_WINDOW, "L went past stretch");

    s.feed([RawEvent::Down(Btn::R1)], 200);
    assert_eq!(
        s.app().source_rect(),
        FIT_WINDOW,
        "R did not step back to the fitted picture"
    );
    assert_eq!(pad(&s) & ButtonMask::R, 0, "R reached the game as well");
    s.feed([RawEvent::Up(Btn::R1)], 220);

    s.feed([RawEvent::Down(Btn::R1)], 240);
    s.feed([RawEvent::Up(Btn::R1)], 260);
    assert_eq!(
        s.app().source_rect(),
        WHOLE_TEXTURE,
        "a second R did not give it back"
    );
    s.feed([RawEvent::Down(Btn::R1)], 280);
    s.feed([RawEvent::Up(Btn::R1)], 290);
    assert_eq!(
        s.app().source_rect(),
        WHOLE_TEXTURE,
        "R went past actual size"
    );

    let d = common::tmp_root_with_carts(&["Emerald", "Zzz"]);
    let mut s = session_playing(d.path());
    s.feed([RawEvent::Down(Btn::L1)], 100);
    assert_ne!(pad(&s) & ButtonMask::L, 0, "the GBA lost its own L");
    s.feed([RawEvent::Down(Btn::R1)], 120);
    assert_ne!(pad(&s) & ButtonMask::R, 0, "the GBA lost its own R");
    assert_eq!(
        s.app().source_rect(),
        WHOLE_TEXTURE,
        "a GBA picture moved when its shoulders were pressed"
    );
}

#[test]
fn the_picture_mode_is_remembered_per_cart() {
    let d = common::tmp_root_with_gb_carts(&["Tetris", "Zzz"]);
    let mut s = session_playing(d.path());
    s.feed([RawEvent::Down(Btn::L1)], 100);
    s.feed([RawEvent::Up(Btn::L1)], 120);
    assert_eq!(video_mode_for(d.path(), "Tetris"), VideoMode::Fit);
    assert_eq!(
        video_mode_for(d.path(), "Zzz"),
        VideoMode::Actual,
        "the press was recorded against another cart as well"
    );
    drop(s);

    let mut s = session_playing(d.path());
    assert_eq!(
        s.app().source_rect(),
        FIT_WINDOW,
        "the cart did not come back fitted"
    );
    s.feed([RawEvent::Down(Btn::R1)], 100);
    assert_eq!(video_mode_for(d.path(), "Tetris"), VideoMode::Actual);
    assert_eq!(s.app().source_rect(), WHOLE_TEXTURE);
}

#[test]
fn only_the_fitted_picture_is_softened() {
    let d = common::tmp_root_with_gb_carts(&["Tetris", "Zzz"]);
    let mut s = session_playing(d.path());
    let mut seen = Vec::new();
    for at in [100, 200, 300] {
        seen.push((s.app().source_rect(), s.app().softens_picture()));
        s.feed([RawEvent::Down(Btn::L1)], at);
        s.feed([RawEvent::Up(Btn::L1)], at + 20);
    }
    assert_eq!(
        seen,
        [
            (WHOLE_TEXTURE, false),
            (FIT_WINDOW, true),
            (GB_WINDOW, false)
        ]
    );
}

#[test]
fn the_fitted_picture_keeps_its_bezel_and_the_stretched_one_drops_it() {
    let d = common::tmp_root_with_gb_carts(&["Tetris", "Zzz"]);
    let mut s = session_playing(d.path());
    let bezel = TexId::from_raw(990);
    s.app_mut().set_game_bezel(Some(bezel));
    assert_eq!(s.app().shown_game_bezel(), Some(bezel));
    s.feed([RawEvent::Down(Btn::L1)], 100);
    s.feed([RawEvent::Up(Btn::L1)], 120);
    assert_eq!(s.app().source_rect(), FIT_WINDOW);
    assert_eq!(s.app().shown_game_bezel(), Some(bezel));
    s.feed([RawEvent::Down(Btn::L1)], 140);
    assert_eq!(s.app().source_rect(), GB_WINDOW);
    assert_eq!(
        s.app().shown_game_bezel(),
        None,
        "the bezel is drawn over the stretched picture"
    );
}

#[test]
fn a_gba_cart_saved_as_fit_still_shows_the_whole_picture() {
    let d = common::tmp_root_with_carts(&["Emerald", "Zzz"]);
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(d.path().join(VIDEO_MODE_FILE), "Emerald = fit\n").unwrap();
    let s = session_playing(d.path());
    assert_eq!(s.app().source_rect(), WHOLE_TEXTURE);
    assert!(!s.app().softens_picture(), "a GBA picture was softened");
}
