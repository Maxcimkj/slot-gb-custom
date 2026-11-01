//! Fork: Shimmerless Grid as a Shader choice for Game Boy and Game Boy Color, never for GBA.
//! `on_carousel_with`, `press` and `open_at` are copies of upstream's in tests/quick_menu.rs.

mod common;

use common::{app_booting_at, tmp_root_with_carts, Clock, CLOCK_IS_SET};
use slot::app::{App, Phase};
use slot_input::{Action, Btn};
use slot_store::{read_slot_state, write_slot_state, Shader, SlotState};
use slot_ui::{QuickRow, QuickValue};
use tempfile::TempDir;

fn on_carousel_with(state: SlotState) -> (TempDir, App, Clock) {
    let d = tmp_root_with_carts(&["Emerald", "Fusion"]);
    write_slot_state(
        d.path(),
        &SlotState {
            clock_set: true,
            ..state
        },
    )
    .expect("write slot.state");
    let (a, clock) = app_booting_at(d.path(), CLOCK_IS_SET);
    assert!(
        matches!(a.phase(), Phase::Shelf),
        "not on the carousel: {:?}",
        a.phase()
    );
    (d, a, clock)
}

fn press(a: &mut App, btn: Btn) {
    a.apply(Action::GbaDown(btn));
    a.apply(Action::GbaUp(btn));
}

fn open_at(a: &mut App, row: QuickRow) {
    a.apply(Action::QuickMenu);
    let mut path = vec![row];
    while let Some(parent) = path.last().and_then(|r| r.parent()) {
        path.push(parent);
    }
    for (depth, step) in path.iter().rev().enumerate() {
        if depth > 0 {
            press(a, Btn::A);
        }
        for _ in 0..step.position(&a.hidden_rows()) {
            press(a, Btn::Down);
        }
    }
    assert_eq!(a.quick_menu(), Some(row), "the bar never reached {row:?}");
}

#[test]
fn the_game_boy_shader_rows_step_to_shimmerless_grid_after_shimmerless_and_wrap_to_off() {
    let gb: fn(&SlotState) -> Shader = |s| s.shader_gb;
    let gbc: fn(&SlotState) -> Shader = |s| s.shader_gbc;
    for (row, saved) in [(QuickRow::GbShader, gb), (QuickRow::GbcShader, gbc)] {
        let (d, mut a, _) = on_carousel_with(SlotState::default());
        open_at(&mut a, row);
        assert_eq!(a.quick_value(row), Some(QuickValue::Simpletex));
        for (btn, want, shown) in [
            (Btn::Right, Shader::Shimmerless, QuickValue::Shimmerless),
            (
                Btn::Right,
                Shader::ShimmerlessGrid,
                QuickValue::ShimmerlessGrid,
            ),
            (Btn::Right, Shader::Off, QuickValue::Off),
            (
                Btn::Left,
                Shader::ShimmerlessGrid,
                QuickValue::ShimmerlessGrid,
            ),
        ] {
            press(&mut a, btn);
            assert_eq!(a.quick_value(row), Some(shown), "{row:?} {btn:?}");
            assert_eq!(saved(&read_slot_state(d.path())), want, "{row:?} {btn:?}");
        }
    }
}

#[test]
fn the_gba_shader_row_never_offers_shimmerless_grid() {
    let (d, mut a, _) = on_carousel_with(SlotState::default());
    open_at(&mut a, QuickRow::GbaShader);
    for _ in 0..Shader::GBA.len() * 2 {
        press(&mut a, Btn::Right);
        assert_ne!(
            a.quick_value(QuickRow::GbaShader),
            Some(QuickValue::ShimmerlessGrid)
        );
        assert_ne!(
            read_slot_state(d.path()).shader_gba,
            Shader::ShimmerlessGrid
        );
    }
}

#[test]
fn shimmerless_grid_is_saved_by_name_and_read_back() {
    assert_eq!(Shader::ShimmerlessGrid.name(), "shimmerless-grid");
    assert_eq!(
        Shader::parse("shimmerless-grid"),
        Some(Shader::ShimmerlessGrid)
    );
    let (d, a, _) = on_carousel_with(SlotState {
        shader_gb: Shader::ShimmerlessGrid,
        shader_gbc: Shader::ShimmerlessGrid,
        ..SlotState::default()
    });
    let text = std::fs::read_to_string(d.path().join("Config/slot.state")).expect("slot.state");
    assert!(text.contains("shader_gb=shimmerless-grid\n"), "{text}");
    assert!(text.contains("shader_gbc=shimmerless-grid\n"), "{text}");
    let s = read_slot_state(d.path());
    assert_eq!(
        (s.shader_gb, s.shader_gbc),
        (Shader::ShimmerlessGrid, Shader::ShimmerlessGrid)
    );
    assert_eq!(
        a.quick_value(QuickRow::GbShader),
        Some(QuickValue::ShimmerlessGrid)
    );
}

#[test]
fn a_gba_shader_saved_as_shimmerless_grid_falls_back_to_the_default() {
    let (d, _a, _) = on_carousel_with(SlotState {
        shader_gba: Shader::ShimmerlessGrid,
        ..SlotState::default()
    });
    assert_eq!(
        read_slot_state(d.path()).shader_gba,
        SlotState::default().shader_gba
    );
}

#[test]
fn the_shader_shortcut_reaches_shimmerless_grid_on_a_game_boy_color_game() {
    let d = common::tmp_root_with_gbc_carts(&["Crystal", "Oracle"]);
    let (mut s, _motor) = common::session_with_platform(d.path());
    s.app_mut().apply(Action::Insert);
    s.app_mut().apply(Action::ShaderNext);
    assert_eq!(s.app().screen_shader(), Shader::Shimmerless);
    s.app_mut().apply(Action::ShaderNext);
    assert_eq!(s.app().screen_shader(), Shader::ShimmerlessGrid);
    assert_eq!(
        read_slot_state(d.path()).shader_gbc,
        Shader::ShimmerlessGrid
    );
    s.app_mut().apply(Action::ShaderNext);
    assert_eq!(s.app().screen_shader(), Shader::Off);
}
