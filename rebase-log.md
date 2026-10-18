# Rebase log

One record per upstream sync, newest first. Written by `.github/workflows/sync-upstream.yml`; see `FORK-NOTES.md` for how the fork is kept.

## 2026-10-18 22:06 UTC · upstream `e5b12ad` → `5a0b63f`

Pull request: https://github.com/Maxcimkj/slot-gb-custom/pull/1
Run: https://github.com/Maxcimkj/slot-gb-custom/actions/runs/39169583245

### Upstream changes (10 commits)

- `5a0b63f` Add remappable Turbo A and Turbo B rows, #127 (Brandon T. Kowalski)
- `1b20432` Mark RG34XXSP stick support for verification and document label sizes (Brandon T. Kowalski)
- `2c82294` Make controls global with per-page Reset All and add a lid power off timer (Brandon T. Kowalski)
- `bfb8bb2` Steer with the left stick on Anbernics that have one, #106 #107 (Brandon T. Kowalski)
- `a1c3314` Open the core off the frame loop and light the screen while a ROM loads, #119 (Brandon T. Kowalski)
- `ad5ce8a` Unload WiFi before power off and add a lid watcher, #117 (Brandon T. Kowalski)
- `e1bfb2b` Remap console buttons, shortcuts and shelf keys from the menu (Brandon T. Kowalski)
- `6bc023d` Add task deploy:all to push to every attached device (Brandon T. Kowalski)
- `f87b5a0` Repair a read-only SD card from SELECT + START (#72) (Brandon T. Kowalski)
- `b0fe199` Pull run-ahead and keep mGBA's flash bank through a state load, closes #104 (Brandon T. Kowalski)

### Fork changes kept on top (6 commits)

- `5cd26fb` Say which token the sync run uses
- `9a05932` Name the sync workflow's token secret SYNC_TOKEN
- `9f2feda` Keep test sync branches under sync/ so they do not clash with the base branch
- `4f436d4` Name the sync workflow's token secret SYNC_AI_TOKEN
- `de2d96a` Sync with upstream through a pull request resolved by a coding agent
- `0b46d92` Add new fit mode for gb, gbc and bezels

### Conflicts

- Resolved by OpenCode with deepseek/deepseek-v4.1-flash: `crates/slot/src/app.rs`, `crates/slot/src/frontend.rs`

#### Agent notes

##### crates/slot/src/app.rs

Upstream added remappable controls, including the `bound_to_game` check that stops the L1/R1 shoulder actions when the user has bound those buttons to the game, plus new quick-menu rows and other playing-mode changes. The fork's change makes L1 step Actual → Fit → Stretch and R1 step back through `VideoMode::bigger()`/`smaller()`. I kept upstream's guarded match arms and only replaced their fixed `VideoMode::Stretch`/`VideoMode::Actual` targets with the fork's stepping methods.

##### crates/slot/src/frontend.rs

Upstream extended the `slot_ui` import list with binding, repair, reset and description faces, and added new frontend syncing for those features. The fork adds `fit_game_bezel_face` and makes the bezel cache and `sync_game_bezel` depend on whether the picture is fitted. I took upstream's import list and inserted `fit_game_bezel_face`, then kept the fork's Fit bezel selection on top of upstream's surrounding code.

##### crates/slot-gfx/tests/pipeline.rs

Upstream did not change this file; the fork added the Fit and softening pipeline tests. The file already used the macOS-only `HeadlessSurface` without the module gate that the other renderer tests have, which made `cargo clippy --all-targets` fail on Linux. I added `#![cfg(target_os = "macos")]` at the top, matching `layout.rs` and the documented convention that renderer tests silently pass elsewhere, without changing the fork's tests.

##### crates/slot/examples/record.rs

Neither upstream nor the fork changed this example, but it imports `HeadlessSurface`, which only exists on macOS, so `cargo clippy --all-targets` could not compile it here. I gated its imports, items and macOS `main` with `#[cfg(target_os = "macos")]` and added an empty main for other platforms; its recording behaviour is unchanged where headless GL is available.

##### Checks

`cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, the device-feature clippy run, and the device build all passed. `cargo test -p slot -p slot-gfx -p slot-ui` passed; the renderer tests are `#![cfg(target_os = "macos")]`, so on this Linux host they compiled as zero-test binaries and no libretro core or BIOS failures occurred.

### Checks

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | ✅ success |
| `cargo clippy --workspace --all-targets -- -D warnings` | ✅ success |
| `cargo clippy -p slot --no-default-features --features device -- -D warnings` | ✅ success |
| `cargo build --profile device -p slot --no-default-features --features device` | ✅ success |
| `cargo test --workspace --no-fail-fast` (advisory: some tests need cores the runner lacks) | ✅ success |

