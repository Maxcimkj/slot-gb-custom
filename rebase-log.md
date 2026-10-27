# Rebase log

One record per upstream sync, newest first. Written by `.github/workflows/sync-upstream.yml`; see `FORK-NOTES.md` for how the fork is kept.

## 2026-10-27 10:45 UTC · upstream `20703ed` → `db1e370`

Pull request: https://github.com/Maxcimkj/slot-gb-custom/pull/11
Run: https://github.com/Maxcimkj/slot-gb-custom/actions/runs/40489569087

### Upstream changes (3 commits)

- `db1e370` Give GBA Video its own shelf and keep the Game Boy Player off for it, closes #140 (Brandon T. Kowalski)
- `aa8bfab` Close the audio device before powering off or restarting, closes #142 (Brandon T. Kowalski)
- `ab67ece` Set the jumped-to letter at the title size, closes #151 (Brandon T. Kowalski)

### Fork changes kept on top (5 commits)

- `b9adfc6` Fit: upstream's Grid, softening only in Fit, Fit decided from a seated cart
- `35cbb3f` Fit: Actual-style bezels, even grid, and all fork code in fit*.rs
- `8575439` Make README.md the fork's own and keep it on every upstream sync
- `c59df76` Take release notes for -gp-custom tags from the upstream version's section
- `0b46d92` Add new fit mode for gb, gbc and bezels

### Conflicts

None. Upstream merged cleanly into the fork.

#### Agent notes

The merge of upstream `db1e370` into the fork was clean: no file carried conflict markers, so every
hook in FORK-NOTES.md section 3 was kept by git without hand editing. Upstream's three commits add
`Platform::Gbav` (GBA Video) with a `hardware()` helper, close the audio device on restart and
poweroff, and set the jumped-to letter at the title size. Two files needed hand work for the build.

##### crates/slot/src/fit.rs

Upstream added a fourth `Platform` variant, `Gbav` (GBA Video), and a `Platform::hardware()` method
that folds it back onto `Platform::Gba`. The fork's `fits` decided Fit from `platform !=
Platform::Gba`, which would have fitted and softened a GBA Video cart even though FORK-NOTES.md says
GBA has no Fit. I changed it to `platform.hardware() != Platform::Gba` so Gbav behaves like Gba, and
added `Platform::Gbav` to the list of platforms the `the_crop_and_the_fitted_check_agree` test
covers, so it still checks every platform. The assertion itself is unchanged.

##### crates/slot-gfx/tests/pipeline.rs

Upstream's `pipeline.rs` test imports `slot_gfx::HeadlessSurface`, which upstream gates behind
`#[cfg(target_os = "macos")]`, so the target does not compile on this Linux runner. The fork's own
`fit_pipeline.rs` and upstream's `layout.rs` are both macOS-gated, and the fork's `pipeline.rs` had
the same gate before fork commit `35cbb3f` removed it. I re-added `#![cfg(target_os = "macos")]` at
the top so `fork-clippy` (which builds every test target) compiles; no upstream code or assertion
changed. This is a build fix, not a formatting or lint change.

##### Checks

- `python3 .github/sync/sync_upstream.py fork-fmt` and `fork-fmt --check`: passed (9 fork files).
- `python3 .github/sync/sync_upstream.py fork-clippy`: passed, no compile errors and no lints in the
  fork's files.
- `python3 .github/sync/sync_upstream.py fork-clippy device`: passed.
- `cargo build --profile device --target-dir target-device -p slot --no-default-features --features
  device`: passed.
- `python3 .github/sync/sync_upstream.py fork-tests`: every Linux Fit test ran and passed; the
  macOS-only `fit_pipeline`, `fit_colour` and `fit_render_bezel` tests were skipped as expected.
- `cargo test -p slot-gfx --lib --tests`: passed.
- `cargo test -p slot --lib --tests`: passed. Tests that need a real mgba core fall back to the mock
  core on this machine; none failed for a missing core, BIOS or display.

### Checks

| Check | Result |
|---|---|
| `sync_upstream.py fork-fmt --check`: rustfmt on the fork's own files | ✅ success |
| `sync_upstream.py fork-clippy`: builds everything, lints the fork's own files | ✅ success |
| `sync_upstream.py fork-clippy device`: the same for the device build | ✅ success |
| `cargo build --profile device -p slot --no-default-features --features device` | ✅ success |
| `sync_upstream.py fork-tests`: the fork's Fit tests (FORK-NOTES.md section 4) | ✅ success |
| `cargo test --workspace --no-fail-fast --lib --bins --tests` (advisory: some tests need cores the runner lacks) | ✅ success |

## 2026-10-23 10:17 UTC · upstream `91499a9` → `20703ed`

Pull request: https://github.com/Maxcimkj/slot-gb-custom/pull/6
Run: https://github.com/Maxcimkj/slot-gb-custom/actions/runs/39936420802

### Upstream changes (20 commits)

- `20703ed` v1.9.0 CHANGELOG.md (Brandon T. Kowalski)
- `77a1970` Install a boot logo from Config exactly as it is (Brandon T. Kowalski)
- `76237f2` Add screenshots with a gallery, zoom and multi-select, closes #94 (Brandon T. Kowalski)
- `22f1ec2` Let GBA take bezels from the card on 4:3 panels (Brandon T. Kowalski)
- `a7ec171` Give the footer room above it (Brandon T. Kowalski)
- `d969e33` Point the undo steps at the in-game menu (Brandon T. Kowalski)
- `58f4b10` Proofread the docs, stub the Screenshots page and eject the cart from the home page (Brandon T. Kowalski)
- `5775326` Add the slot footer, the Documentation cart and the human-edited Guide (Brandon T. Kowalski)
- `64c4147` Offer Undo Save and Undo Load in the in-game menu (Brandon T. Kowalski)
- `0dfb08c` Draw the About sticker's model in GAMECUBEN from BASEOS_MODEL (Brandon T. Kowalski)
- `47bd0e6` Give each card repair step a time limit (Brandon T. Kowalski)
- `79bc012` Let the shelf come up when label caching stops moving (Brandon T. Kowalski)
- `aee305c` Add a site footer and polish the docs (Brandon T. Kowalski)
- `25315de` Stop waiting on a core that does not answer (Brandon T. Kowalski)
- `55cf31b` Start on the shelf when a cart took slot down while loading (Brandon T. Kowalski)
- `f66497e` Write the in-game save every minute and the resume every five (Brandon T. Kowalski)
- `1edb9ed` Sync the card after each save and each new log line (Brandon T. Kowalski)
- `4e74bdd` Refuse an empty state instead of handing it to the core (Brandon T. Kowalski)
- `7554adc` Rebuild the site on Zensical and add a Guide section (Brandon T. Kowalski)
- `421f11f` Use the CHANGELOG section for the version as the release notes (Brandon T. Kowalski)

### Fork changes kept on top (1 commits)

- `0b46d92` Add new fit mode for gb, gbc and bezels

### Conflicts

- `site/index.html`: Upstream deleted this file, so the fork's edits to it were dropped (upstream wins).

#### Agent notes

##### Sync notes

Merged upstream `20703ed` into the fork (`36e6424`) on branch `sync/upstream-20703ed`. The
merge was clean: no file contained conflict markers, so every fork change in `FORK-NOTES.md`
section 3 was kept by git without hand editing. The fork's Fit feature, bezel art, softening
shader and Fit tests are all present, and the full check set passes. The only hand edits were
three clippy lints in an upstream test and the re-application of the fork's L1/R1 wording to
upstream's rebuilt site.

##### crates/slot-ui/tests/shot_viewer.rs

Upstream added this screenshot-gallery test in `76237f2`, and it was new to the merge. Clippy
1.96 under `-D warnings` rejected three lints in it: a redundant guard, a manual
`Range::contains`, and a `useless_vec`. The fork does not otherwise touch the file. I applied
the three mechanical clippy fixes and changed no assertion.

##### site/index.html

Upstream `7554adc` rebuilt the site on Zensical and deleted `site/index.html`, so the fork's
two L1/R1 wording changes there no longer apply to any file. Following `FORK-NOTES.md` rule 3 I
kept upstream's deletion and moved the fork's wording to the new Markdown guide instead, as the
table's "take upstream's markup, then re-apply the wording" asks.

##### site/docs/tour.md

Upstream's new guide describes L1 as stretching and R1 as restoring, which matches upstream's
two-mode behaviour. The fork's Fit mode makes L1 step through Actual, then full height, then
stretched. I re-applied the fork's wording to upstream's new line so the tour describes the
three-step cycle.

##### site/docs/guide/controls.md

Upstream's new button reference lists L1/R1 as "Stretch to fill the screen / original size".
The fork's change makes them a bigger/smaller pair over three modes. I replaced the description
with the fork's "Bigger: original size, full height, stretched / smaller again" wording on
upstream's row, leaving the surrounding markup alone.

##### site/docs/guide/screen.md

Upstream's new Screen guide says L1 stretches, R1 restores, and bezels are hidden while
stretched. In the fork the full-height Fit step keeps its bezel and only Stretch hides it. I
kept upstream's prose and adjusted it to the fork's behaviour, including that the bezel is
hidden only when the game is stretched across the screen.

##### Formatting pass

`cargo fmt --all` was required and reformatted upstream's newest code, which this toolchain's
rustfmt wraps differently. Every change below is pure line wrapping or argument layout with no
semantic effect; the fork does not touch these files. They were reformatted so
`cargo fmt --all -- --check` passes.

##### crates/slot-store/tests/bindings.rs

Upstream changed this screenshot-store test in the screenshots work; the fork does not touch it.
`cargo fmt --all` only rewrapped its long lines, with nothing else changed.

##### crates/slot-store/tests/shots.rs

Upstream changed this screenshot-store test in the screenshots work; the fork does not touch it.
`cargo fmt --all` only rewrapped its long lines, with nothing else changed.

##### crates/slot-ui/src/plate.rs

Upstream rewrote the plate module for the screenshots feature; the fork does not touch it.
`cargo fmt --all` only reflowed one `blit` call's arguments, with nothing else changed.

##### crates/slot-ui/src/shot_viewer.rs

Upstream added this viewer module in `76237f2`; the fork does not touch it. `cargo fmt --all`
only rewrapped several long calls and closures, with nothing else changed.

##### crates/slot/src/shot_loader.rs

Upstream changed this screenshot loader; the fork does not touch it. `cargo fmt --all` only
rewrapped a long expression, with nothing else changed.

##### crates/slot/src/shot_writer.rs

Upstream changed this screenshot writer; the fork does not touch it. `cargo fmt --all` only
rewrapped its long calls, with nothing else changed.

##### crates/slot/tests/emu.rs

Upstream changed this test file over the sync range; the fork does not touch it. `cargo fmt
--all` only rewrapped one `assert!`, with nothing else changed.

##### crates/slot/tests/flush.rs

Upstream changed this test file over the sync range; the fork does not touch it. `cargo fmt
--all` only rewrapped one `assert_eq!`, with nothing else changed.

##### crates/slot/tests/in_game_menu.rs

Upstream changed this test file over the sync range; the fork does not touch it. `cargo fmt
--all` only rewrapped its long lines, with nothing else changed.

##### crates/slot/tests/label_cache.rs

Upstream changed this test file over the sync range; the fork does not touch it. `cargo fmt
--all` only rewrapped a command builder chain and two assertions, with nothing else changed.

##### crates/slot/tests/loading_mark.rs

Upstream changed this test file over the sync range; the fork does not touch it. `cargo fmt
--all` only reflowed a `persist::flush` call and an `assert!`, with nothing else changed.

##### crates/slot/tests/quick_menu.rs

Upstream changed this test file over the sync range; the fork does not touch it. `cargo fmt
--all` only rewrapped its long lines, with nothing else changed.

##### crates/slot/tests/render_gallery_stress.rs

Upstream added this screenshot stress test; the fork does not touch it. `cargo fmt --all` only
rewrapped its long calls and assertions, with nothing else changed.

##### crates/slot/tests/render_screenshot.rs

Upstream added this screenshot render test; the fork does not touch it. `cargo fmt --all` only
rewrapped its helpers and assertions, with nothing else changed.

##### crates/slot/tests/screenshot.rs

Upstream added this screenshot test; the fork does not touch it. `cargo fmt --all` only
rewrapped its long calls and assertions, with nothing else changed.

##### crates/slot/tests/shot_loader.rs

Upstream changed this loader test over the sync range; the fork does not touch it. `cargo fmt
--all` only rewrapped its long lines, with nothing else changed.

##### Checks

- `cargo fmt --all` ran and `cargo fmt --all -- --check` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes after the three
  `crates/slot-ui/tests/shot_viewer.rs` lint fixes.
- `cargo clippy -p slot --no-default-features --features device -- -D warnings` passes.
- `cargo build --profile device --target-dir target-device -p slot --no-default-features
  --features device` passes.
- `python3 .github/sync/sync_upstream.py fork-tests` passes: the five `slot/tests/app.rs` Fit
  tests, the two `slot/tests/fit_softening.rs` tests and the `slot-ui/tests/fit_bezel.rs` test
  ran and passed. The macOS-only `slot-gfx/tests/pipeline.rs`, `slot-gfx/tests/fit_colour.rs`
  and `slot/tests/render_fit_bezel.rs` tests were skipped because this machine has no headless
  GL.
- `cargo test -p slot-ui`, `cargo test -p slot` and `cargo test -p slot-gfx` pass: every result
  line is `ok`, zero failures. Tests that need a real libretro core fall back to the mock core
  on this machine, as before.

### Checks

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | ✅ success |
| `cargo clippy --workspace --all-targets -- -D warnings` | ✅ success |
| `cargo clippy -p slot --no-default-features --features device -- -D warnings` | ✅ success |
| `cargo build --profile device -p slot --no-default-features --features device` | ✅ success |
| `sync_upstream.py fork-tests`: the fork's Fit tests (FORK-NOTES.md section 4) | ✅ success |
| `cargo test --workspace --no-fail-fast` (advisory: some tests need cores the runner lacks) | ✅ success |

## 2026-10-22 10:22 UTC · upstream `842649e` → `91499a9`

Pull request: https://github.com/Maxcimkj/slot-gb-custom/pull/5
Run: https://github.com/Maxcimkj/slot-gb-custom/actions/runs/39761202570

### Upstream changes (7 commits)

- `91499a9` Let Fast Forward be set to Off (#123) (Brandon T. Kowalski)
- `0d36d9d` Ask mGBA for the state size again once the game settles on its save chip, closes #126 (Brandon T. Kowalski)
- `07183c8` Leave the audio device closed at boot so the BaseOS startup sound can play, closes #135 (Brandon T. Kowalski)
- `af9d569` Mark the RG34XXSP left stick as verified (Brandon T. Kowalski)
- `9e545a3` Keep the shelf drawn while a held letter jump outruns the scroll (Brandon T. Kowalski)
- `412f694` Add Load State on Insert and rename Auto Save on Eject to Save State on Eject, closes #130 (Brandon T. Kowalski)
- `1b2001e` Build mGBA with the upstream fix for MBC2 games crashing on their first save RAM read, closes #137 (Brandon T. Kowalski)

### Fork changes kept on top (1 commits)

- `0b46d92` Add new fit mode for gb, gbc and bezels

### Conflicts

None. Upstream merged cleanly into the fork.

#### Agent notes

##### Sync notes

Merged upstream `91499a9` into the fork (`ffe6dc6`) on branch `sync/upstream-91499a9`.
The merge was clean: no file contained conflict markers, so no fork change had to be
re-applied against upstream's code. Every fork feature from `FORK-NOTES.md` section 3 is
present in the result, and the full check set passes.

##### crates/slot-ui/tests/letters.rs

Upstream added `holding_a_letter_jump_never_empties_the_shelf` (commit `9e545a3`) with its
`assert!(covered, ...)` on one long line. The fork does not touch this file. Running
`cargo fmt --all` wrapped that assertion onto several lines, which is the only working-tree
change; the test and what it asserts are unchanged.

##### Checks

- `cargo fmt --all` passed (it rewrapped `crates/slot-ui/tests/letters.rs`; nothing else).
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo clippy -p slot --no-default-features --features device -- -D warnings` passed.
- `cargo build --profile device --target-dir target-device -p slot --no-default-features --features device` passed.
- `python3 .github/sync/sync_upstream.py fork-tests` passed: the five `slot/tests/app.rs`
  Fit tests, the two `slot/tests/fit_softening.rs` tests and the `slot-ui/tests/fit_bezel.rs`
  test all ran and passed. The macOS-only `slot-gfx/tests/pipeline.rs`,
  `slot-gfx/tests/fit_colour.rs` and `slot/tests/render_fit_bezel.rs` tests were skipped as
  the runner has no headless GL.
- `cargo test -p slot-gfx`, `cargo test -p slot-ui` and `cargo test -p slot --no-fail-fast`
  passed: every test result line was `ok`, zero failures. Tests that need a real libretro
  core fall back to the mock core on this machine, as before.

### Checks

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | ✅ success |
| `cargo clippy --workspace --all-targets -- -D warnings` | ✅ success |
| `cargo clippy -p slot --no-default-features --features device -- -D warnings` | ✅ success |
| `cargo build --profile device -p slot --no-default-features --features device` | ✅ success |
| `sync_upstream.py fork-tests`: the fork's Fit tests (FORK-NOTES.md section 4) | ✅ success |
| `cargo test --workspace --no-fail-fast` (advisory: some tests need cores the runner lacks) | ✅ success |

## 2026-10-21 10:17 UTC · upstream `f4b7bc3` → `842649e`

Pull request: https://github.com/Maxcimkj/slot-gb-custom/pull/4
Run: https://github.com/Maxcimkj/slot-gb-custom/actions/runs/39582423050

### Upstream changes (11 commits)

- `842649e` Spread Joy. (Brandon T. Kowalski)
- `0ceac65` update changelog for v1.8.0 (Brandon T. Kowalski)
- `fdfc5d1` Ask before deleting a save state, with X to delete and B to keep it (Brandon T. Kowalski)
- `b4a54d3` End the in-game Screen page with brightness and blue light, three rows at a time, under a scroll bar (Brandon T. Kowalski)
- `af9ff09` Flag only a single on START against a START chord, not one on its second button (Brandon T. Kowalski)
- `0fa8ecc` Say RECORDING while a control waits for a button (Brandon T. Kowalski)
- `6d8f8db` Put brightness, blue light and one live Palette row in the menus (Brandon T. Kowalski)
- `e22101c` Wait out the WiFi unload in the repair test so its fake sleep cannot race it (Brandon T. Kowalski)
- `b25c7a2` Let the card repair take as many fsck passes as FAT damage needs (Brandon T. Kowalski)
- `c5de4cc` Keep the SD card safe through crashes, power offs and failed reads (Brandon T. Kowalski)
- `ea438a0` Print the release version on the About sticker (Brandon T. Kowalski)

### Fork changes kept on top (1 commits)

- `0b46d92` Add new fit mode for gb, gbc and bezels

### Conflicts

- Resolved by OpenCode with deepseek/deepseek-v4.1-flash: `crates/slot/src/frontend.rs`

#### Agent notes

##### crates/slot/src/frontend.rs

Upstream added the save-state delete UI to this file, which pulled `delete_legends` into the `slot_ui` import list; the rest of its changes here were unrelated to the fork. The fork adds `fit_game_bezel_face` to the same import list and uses it, along with a `fits` flag in `BezelSync`, to draw the Fit bezel and hide card bezels while fitted. The conflict was only in the import block, so I kept upstream's full list (including `delete_legends`) and inserted `fit_game_bezel_face` in alphabetical order; `cargo fmt --all` reflowed it to match. The fork's `BezelSync` field, `set_soft_picture` call and `(pick, fits)` match were already present and are unchanged.

##### Checks

`cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo clippy -p slot --no-default-features --features device -- -D warnings`, the device build, `python3 .github/sync/sync_upstream.py fork-tests` and `cargo test -p slot` all passed. The macOS-only Fit render tests were skipped on this Linux machine as expected.

### Checks

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | ✅ success |
| `cargo clippy --workspace --all-targets -- -D warnings` | ✅ success |
| `cargo clippy -p slot --no-default-features --features device -- -D warnings` | ✅ success |
| `cargo build --profile device -p slot --no-default-features --features device` | ✅ success |
| `sync_upstream.py fork-tests`: the fork's Fit tests (FORK-NOTES.md section 4) | ✅ success |
| `cargo test --workspace --no-fail-fast` (advisory: some tests need cores the runner lacks) | ✅ success |

## 2026-10-20 10:17 UTC · upstream `5a0b63f` → `f4b7bc3`

Pull request: https://github.com/Maxcimkj/slot-gb-custom/pull/3
Run: https://github.com/Maxcimkj/slot-gb-custom/actions/runs/39405419565

### Upstream changes (7 commits)

- `f4b7bc3` Resolve every control through one table with chords, contexts and clash warnings (Brandon T. Kowalski)
- `ee19b7a` Size wallpapers and bezels to the panel and document art sizes, #122 (Brandon T. Kowalski)
- `0b736f1` Keep tests silent by default and let the run tasks play sound (Brandon T. Kowalski)
- `8d84346` Expect at least one link legend now that Tetris cannot switch modes (Brandon T. Kowalski)
- `be8acb6` Box the eighth hash character on the sticker instead of a dirty bit (Brandon T. Kowalski)
- `211c829` Offer the link mode switch only on gpSP for carts with both links, #131 (Brandon T. Kowalski)
- `9d2bc24` Lay out every screen from the panel size and trim the link and shortcut legends (Brandon T. Kowalski)

### Fork changes kept on top (1 commits)

- `0b46d92` Add new fit mode for gb, gbc and bezels

### Conflicts

- Resolved by OpenCode with deepseek/deepseek-v4.1-flash: `crates/slot/src/frontend.rs`

#### Agent notes

##### Sync notes

Sync of upstream `f4b7bc3` into the fork. Upstream reworked bezels and wallpapers to
size against the panel instead of the fixed 720x480 canvas (`d9fc...`/`ee19b7a`), flattened the
control table (`f4b7bc3`), and dropped the sticker dirty bit. The fork's Fit mode and its bezel art
were re-applied on top of the new panel-aware bezel code.

##### crates/slot/src/frontend.rs

Three conflicts. Upstream added `quick_warning_value_face` to the `slot_ui` import list, gave
`BezelSync.image` a `(u32, u32)` panel-size field, added `App::game_bezel_size()`, and passed that
size to `default_game_bezel_face`/`game_bezel_face`; it also dropped `set_overlay_slide` and the
`begin_frame(layout)` argument. The fork adds `fit_game_bezel_face` to the imports, a `fits` flag to
the cache key, `compositor.set_soft_picture(app.softens_picture())` in `compose()`, and a
`(pick, fits)` match in `sync_game_bezel` that picks the fit art in Fit and hides a card bezel while
fitted. I kept upstream's import list and added `fit_game_bezel_face`, extended the cache key to
`(Console, BezelPick, (u32, u32), bool)`, and built `shown = (platform, pick, (w, h), fits)` so the
match is on `(&shown.1, shown.3)`, passing `w, h` to the default, fit and card bezel face calls. The
fork's `set_soft_picture` line survived the auto-merge next to upstream's `set_game_source_rect`.

##### crates/slot-ui/src/backdrop.rs

Auto-merged so that upstream's new `panel_bezel` helper and the panel-sized `default_game_bezel_face`
(matching `(w, h)`) sit alongside the fork's fit bezel include and `fit_game_bezel_face`. The fork
function still took only `colour` and hard-coded `art::cover_bytes(png, OUT_W, OUT_H)`, which no
longer matched how bezels are sized. I changed its signature to `fit_game_bezel_face(colour, w, h)`
and cover the art to `(w, h)`, mirroring the non-framed branch of `default_game_bezel_face`. The fit
art is transparent side bars drawn over the full-height picture, so no `panel_bezel` gap logic is
needed; on the 720x480 RG SP this is exactly the fork's old behaviour.

##### crates/slot-ui/tests/fit_bezel.rs

This is the fork's Fit bezel test. It called `fit_game_bezel_face(colour)` and asserted the art is
720x480 with alpha only in the side bars. Because the function now takes the panel size, I updated
only the call's setup to `fit_game_bezel_face(colour, W as u32, H as u32)`; the assertions and the
side-bar bounds are unchanged.

##### Other files formatted by cargo fmt

`crates/slot-ui/src/sticker.rs` and `crates/slot/tests/common/mod.rs` came out of the merge with
formatting `cargo fmt --all` rewrapped (a status-line `c.print` call collapsed onto one line, and an
array literal in a test helper expanded). No logic changed in either.

##### Checks

All required commands passed: `cargo fmt --all`; `cargo clippy --workspace --all-targets -- -D
warnings`; `cargo clippy -p slot --no-default-features --features device -- -D warnings`; `cargo
build --profile device --target-dir target-device -p slot --no-default-features --features device`;
`python3 .github/sync/sync_upstream.py fork-tests` (all present, the three macOS render tests
skipped on Linux). `cargo test -p slot-ui` and `cargo test -p slot` passed in full; the only
non-passing entries in `-p slot` are upstream's two pre-existing `#[ignore]` mario-kart link tests.
No test needed a libretro core, BIOS or display that this machine lacks. The macOS-only tests
(`slot-gfx/tests/pipeline.rs`, `fit_colour.rs`, `slot/tests/render_fit_bezel.rs`) do not compile
here; none of them call the changed `fit_game_bezel_face` signature.

### Checks

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | ✅ success |
| `cargo clippy --workspace --all-targets -- -D warnings` | ✅ success |
| `cargo clippy -p slot --no-default-features --features device -- -D warnings` | ✅ success |
| `cargo build --profile device -p slot --no-default-features --features device` | ✅ success |
| `sync_upstream.py fork-tests`: the fork's Fit tests (FORK-NOTES.md section 4) | ✅ success |
| `cargo test --workspace --no-fail-fast` (advisory: some tests need cores the runner lacks) | ✅ success |

## 2026-10-18 22:22 UTC · upstream `e5b12ad` → `5a0b63f`

Pull request: https://github.com/Maxcimkj/slot-gb-custom/pull/2
Run: https://github.com/Maxcimkj/slot-gb-custom/actions/runs/39170914840

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

### Fork changes kept on top (7 commits)

- `9162c8b` Merge the sync pull request when every required check passes
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
Upstream added a `!self.bound_to_game(Btn::L1)` / `!self.bound_to_game(Btn::R1)` clause to the `slot_owns_the_shoulders()` guard in `Phase::Playing` and reformatted each arm onto two lines. The fork's change is the target of those arms: L1 goes to `self.video_mode.bigger()` and R1 to `self.video_mode.smaller()`, so the shoulder steps through Actual, Fit and Stretch instead of jumping straight to Stretch/Actual. I kept upstream's guard and formatting verbatim and only swapped in the fork's `bigger()`/`smaller()` calls, so neither side's work was dropped. The other fork additions in this file (`fits_picture()`, `softens_picture()` and their neighbours) auto-merged around upstream's new `taken_buttons()`/`bound_to_game()` code.

##### crates/slot/src/frontend.rs
Upstream rewrapped the `slot_ui` import block and added several new names (`menu_detail_face`, `prompt_legends`, `quick_description_face`, `quick_label_face`, `repair_legends`). The fork inserted `fit_game_bezel_face` into that same list for the Fit bezel art. I took upstream's import list as the base and added `fit_game_bezel_face` in alphabetical order after `default_game_bezel_face`, then let `cargo fmt --all` rewrap the block. The fork's other changes in this file (`set_soft_picture` after `set_game_source_rect`, the `(pick, fits)` match in `sync_game_bezel`, the `bool` in the `BezelSync` cache key) auto-merged cleanly and were left as they landed.

##### crates/slot-gfx/tests/pipeline.rs
Upstream did not touch this file; the fork appended two Fit-shader tests (`the_fitted_picture_fills_the_height_with_bars_only_at_the_sides` and `softening_gives_every_fitted_game_boy_row_the_same_weight`). The file imports `HeadlessSurface`, which is `#[cfg(target_os = "macos")]`, but unlike every sibling render test (`layout.rs`, the `slot` and `slot-ui` render tests) it had no `#![cfg(target_os = "macos")]`, so `cargo clippy --workspace --all-targets` failed to compile it on the Linux runner. I added the same `#![cfg(target_os = "macos")]` gate the fork documents for renderer tests, which skips the file off macOS and still runs the Fit tests on a Mac.

##### crates/slot/examples/record.rs
Upstream's `record` example drives `HeadlessSurface`, the macOS-only core-profile context, and the fork does not touch it. Because `cargo clippy --workspace --all-targets` builds examples, this unmodified file broke the Linux host check as well. I gated the `HeadlessSurface` import and the existing `main` behind `#[cfg(target_os = "macos")]` and added a non-macOS `main` that prints one line and exits, with a `cfg_attr` that allows the now-dead helpers off macOS. The example records exactly as before on macOS and compiles everywhere else.

##### Checks
- `cargo fmt --all` and `cargo fmt --all -- --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass after the pipeline.rs and record.rs gates above.
- `cargo clippy -p slot --no-default-features --features device -- -D warnings`: pass.
- `cargo build --profile device --target-dir target-device -p slot --no-default-features --features device`: pass.
- `cargo test -p slot-gfx`: pass (the `pipeline.rs` and `layout.rs` render tests report 0 tests on this Linux host because they are macOS-gated).
- `cargo test -p slot --no-fail-fast`: pass, including the fork's `l_and_r_change_the_mode_on_a_game_boy_cart_and_not_on_a_gba_one`, `the_picture_mode_is_remembered_per_cart`, `only_the_fitted_picture_is_softened`, `the_fitted_picture_keeps_its_bezel_and_the_stretched_one_drops_it` and `a_gba_cart_saved_as_fit_still_shows_the_whole_picture`. The macOS-gated `render_*` tests report 0 tests here; they cannot be verified on this machine.

### Checks

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | ✅ success |
| `cargo clippy --workspace --all-targets -- -D warnings` | ✅ success |
| `cargo clippy -p slot --no-default-features --features device -- -D warnings` | ✅ success |
| `cargo build --profile device -p slot --no-default-features --features device` | ✅ success |
| `cargo test --workspace --no-fail-fast` (advisory: some tests need cores the runner lacks) | ✅ success |

