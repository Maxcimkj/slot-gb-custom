# Fork notes

Context for whoever syncs this fork with upstream, human or AI agent, without the conversations
that produced it. Read this before resolving a conflict.

## 1. What this fork is

- **Fork:** `Maxcimkj/slot-gb-custom`, default branch `main`.
- **Upstream:** `BrandonKowalski/slot`, branch `main`. slot is a Rust frontend for Anbernic H700
  handhelds (RG SP, RG34XX, RG35XX family, RG28XX, RG40XX, CubeXX) that plays GB, GBC and GBA
  games through the mGBA and gpSP libretro cores.
- **Purpose:** carry one feature upstream does not have, a **Fit** picture mode for GB/GBC games,
  plus its bezel art, and publish device zips from the fork. The target device is the RG SP
  (720x480 panel, the same size as slot's canvas).
- **Base:** the fork started from upstream `e5b12ad Add RG35XX-H to the device list, closes #98`
  (after `v1.7.0`). Five upstream syncs have been merged since; the newest upstream in `main` is
  `20703ed v1.9.0 CHANGELOG.md` (sync pull request #6, 2026-10-23). `rebase-log.md` has every
  sync. Fork releases are tagged `v<upstream version>-gp-custom[.N]`; the newest is
  `v1.9.0-gp-custom.2` on `8575439`.
- **Fork commits on `main`** (everything else in `--first-parent main` is a sync merge):

  | Commit | What |
  |---|---|
  | `0b46d92` | The Fit mode, its bezel art and tests (section 3) |
  | `4df38b7` | First upstream merge, by hand, plus the automatic sync workflow and two more Fit test files |
  | `c59df76` (PR #7) | `release.yml` takes the notes for `-gp-custom` tags from the upstream version's section (section 5) |
  | `8575439` | `README.md` becomes the fork's own, with Fit screenshots; the sync always keeps it |
  | `6eedf3d` | Fit bezels get the Actual bezels' grey surround, black frame and inner shadow; `tools/fit_bezel/` builds them |
  | `14e761e` | Grid screen effect at two cells per game pixel in Fit |
  | the commit that adds this row | All fork code moved into `fit*.rs` files; upstream files keep only hooks and upstream's own formatting (section 3) |
- **Sync model:** upstream `main` is merged into the fork's `main` through a pull request, the
  way GitHub's Sync fork → Open pull request does it; history is never rewritten. A coding agent
  resolves the pull request, and the workflow merges it once every required check passes.
  `.github/workflows/sync-upstream.yml` does this daily (05:17 UTC) and on demand:
  1. `.github/sync/sync_upstream.py prepare` copies upstream's `main` to `sync/upstream-<sha>` in
     the fork and opens a pull request from it into `main`; GitHub shows it as conflicted when the
     fork's changes clash. A newer upstream closes an older waiting sync pull request; the same
     upstream is never synced twice while its pull request waits.
  2. It merges `main` into that branch, as GitHub's Resolve conflicts button does. Binary files
     and files one side deleted take upstream's state. Text conflicts go to the OpenCode agent
     running an OpenRouter model, which reads this file, resolves them following section 3
     (upstream wins: take upstream's file and re-add the fork's hooks) and runs the checks of
     section 4 until they pass. It must keep the fork's tests intact (section 4). It may only run
     `cargo` (not `cargo fmt`), read-only `git` and the script's `fork-*` commands, has no web
     access, and may not edit `.github/`, this file, `rebase-log.md` or `README.md`.
     `README.md` is the fork's own: before the agent starts, `prepare` puts back the fork's
     version whatever upstream did to it (`readme_for_fork`), so README conflicts never reach it.
  3. `verify` refuses the result if conflict markers remain or protected files changed. The
     workflow then runs the checks itself: formatting and lints of the fork's own files, a clean
     build (host and device), the device build and the fork's Fit tests by name (section 4) are
     required; the full `cargo test` is advisory because some tests need cores the runner does
     not have.
  4. `finish` commits the merge with a record in `rebase-log.md` (upstream commits, fork commits,
     each conflict, the agent's notes, the check results), pushes the branch, and puts the same
     record in the pull request description. Failed required checks still push the attempt and
     mark the pull request "do not merge".
  5. With every required check passed, the workflow merges the pull request with a merge commit
     (never squash or rebase: the next sync finds what is new from where upstream's history
     joins `main`). Set the repository variable `SYNC_AUTO_MERGE` to `false` to review and merge
     by hand instead; then use "Create a merge commit" too.
  6. After merging, it tags the merge commit as the fork's next release and pushes the tag, which
     starts `release.yml`. The name is `<upstream release>-gp-custom.<n>`, where the upstream
     release is the newest upstream `v*` tag in the merged upstream code and `n` is one past the
     fork's highest for that release: `v1.7.0-gp-custom.2` is followed by `v1.7.0-gp-custom.3`, and
     the first sync after upstream tags `v1.8.0` gives `v1.8.0-gp-custom.1`. **Rule: new upstream
     commits, new tag; none, no tag** (a run with nothing new stops before merging). Syncs merged
     by hand (`SYNC_AUTO_MERGE=false`) and syncs of a test branch are not tagged; tag those by hand.
  Setup it needs: repo secret `OPENROUTER_API_KEY`, repo variable `AGENT_MODEL` (an OpenRouter
  model id such as `vendor/model`, used as `openrouter/<AGENT_MODEL>`), and repo secret `SYNC_TOKEN`, a
  fine-grained token with Contents, Pull requests and Workflows write on this repo, because the
  default token cannot push upstream changes to `.github/workflows/`. Run it from Actions →
  sync-upstream → Run workflow.

## 2. Fork-only files (cannot conflict)

| Path | What |
|---|---|
| `FORK-NOTES.md` | This file |
| `README.md` | The fork's README. Upstream has a file with this name, but the sync always keeps the fork's version (section 1, step 2), so it never conflicts |
| `.github/resources/fork/` | Fit screenshots used by `README.md` |
| `RELEASE.md` | How release zips are built and named. Kept out of `README.md` on purpose so syncs never touch it |
| `rebase-log.md` | One record per automatic upstream sync, newest first, written by the sync workflow |
| `.github/workflows/sync-upstream.yml`, `.github/sync/sync_upstream.py` | The automatic upstream sync pull request, resolved by the OpenCode agent (section 1) |
| `assets/bezel_gb_fit.png`, `assets/bezel_gbc_fit.png` | Built-in Fit bezel art, compiled in |
| `tools/fit_bezel/` | `make_fit_bezels.py` rebuilds the Fit bezel art from the Actual bezels; `label_gb(c).png` are the right-bar labels it places |
| `crates/slot/src/fit.rs` | The Fit mode: `bigger()`/`smaller()` stepping, the Fit crop, which pictures are fitted and softened, and the bezel over a fitted picture |
| `crates/slot-ui/src/fit_bezel.rs` | `fit_game_bezel_face`: the built-in Fit bezel art |
| `crates/slot-gfx/src/fit.rs` | The shader side: `game_frag` patches upstream's `GAME_FRAG` with the softening and the half-cell grid, and `Soft` holds the `u_soft` uniform. Its unit test checks the patch still applies |
| `crates/slot/tests/fit_app.rs` | The Fit mode as the app sees it, including the fork's versions of upstream's two L1/R1 tests |
| `crates/slot/tests/fit_softening.rs` | GBC carts fit and soften exactly like GB carts, softened only while fitted |
| `crates/slot/tests/fit_render_bezel.rs` | Full-frame render of the Fit bezel and of a card bezel in Fit (macOS only) |
| `crates/slot-ui/tests/fit_bezel.rs` | Fit bezel PNGs fill the side bars and only shade the picture's edges |
| `crates/slot-gfx/tests/fit_pipeline.rs` | The fitted picture through the real shader: full height, even rows, half-cell grid (macOS only) |
| `crates/slot-gfx/tests/fit_colour.rs` | Softening colour stripes: every channel even, edges only mix the two colours (macOS only) |

**Rule: every fork Rust file is named `fit*.rs`** (`crates/*/src/fit*.rs`, `crates/*/tests/fit*.rs`).
The sync formats and lints exactly those files (`FORK_RUST` in `sync_upstream.py`), so a new fork
file outside that pattern would go unchecked. Fork logic goes in these files, never in upstream's.

**Rule:** `README.md` belongs to the fork. Upstream's changes to it are never taken: the sync
keeps the fork's version and records that in `rebase-log.md`. It keeps upstream's top part (banner,
What is slot?, What does it look like?, Supported Devices) plus the fork's own section, and points
to upstream's README for the full documentation. If upstream's top part changes in a way that
matters (a new device, new screenshots), copy it over by hand. Do not edit `CHANGELOG.md` in the
fork. If upstream adds a file with one of the other names above, rename ours, not theirs.

## 3. Fork changes to upstream files (conflict hotspots)

The feature started as `0b46d92 Add new fit mode for gb, gbc and bezels` (the tag
`v1.7.0-gp-custom` points to `16c72c7`, an earlier version of it, see section 6). Since then all
of its code has moved into the fork's own `fit*.rs` files (section 2). Upstream files carry only
the hooks below, and otherwise are byte for byte upstream's, formatting included.

### The feature in one paragraph

GB/GBC games had two modes, Actual (exact 3x, 480x432, bezel shown) and Stretch (720x432, bezel
hidden). The fork adds **Fit** between them: full 480 px height at the Game Boy's own 10:9 shape,
about 533x480, with black bars of ~93 px left and right. L1 steps bigger (Actual → Fit → Stretch,
stops at Stretch), R1 steps smaller (Stretch → Fit → Actual, stops at Actual). The mode is saved
per game in `Config/video_mode.txt` as `<rom stem> = fit`. Fit is the only softened mode, using
sharp bilinear, because 3.33x nearest-neighbour makes rows alternate between 3 and 4 lines. With
the Grid screen effect, Fit draws two grid cells per game pixel (about 1.67 px), because one cell
per game pixel at 3.33x gives uneven lines; Actual and Stretch keep upstream's grid. GBA has
no Fit and is never affected, even with `fit` saved for a GBA game.

### The hooks in upstream files

`git diff <upstream> main -- crates/` shows exactly these. When an upstream file conflicts, take
upstream's version and re-add its hooks; if one no longer fits, change the `fit*.rs` file it calls.

| File | Hook | Calls |
|---|---|---|
| `crates/slot/src/lib.rs` | `pub mod fit;` | — |
| `crates/slot/src/video_mode.rs` | `Fit` variant between `Actual` and `Stretch`; `"fit"` arms in `as_str` and `parse`; first lines of `source_rect`: `if let Some(rect) = crate::fit::source_rect(platform, mode) { return rect; }` | `fit::source_rect` returns `None` for every other mode and for GBA, so upstream's own code handles those |
| `crates/slot/src/app.rs` | `fits_picture()` next to `source_rect()` (it reads the private `platform` and `video_mode`); L1 → `set_picture(self.video_mode.bigger())`, R1 → `set_picture(self.video_mode.smaller())` | `fit::fits`; `bigger`/`smaller` are an `impl VideoMode` in `fit.rs`, `softens_picture` an `impl App` there |
| `crates/slot/src/frontend.rs` | `BezelSync.image` key gains `bool`; `let shown = (platform, pick, (w, h), app.fits_picture());`; before upstream's bezel `match`: `let fitted = crate::fit::game_bezel(app, platform, &shown.1, w, h);`, and the match's first arm `_ if fitted.is_some() => fitted.flatten(),`, so a fitted picture never loads the card bezel it hides; in `compose()`, after `set_game_source_rect`: `compositor.set_soft_picture(self.session.app().softens_picture());` | `fit::game_bezel` |
| `crates/slot-ui/src/lib.rs` | `mod fit_bezel;` and `pub use fit_bezel::fit_game_bezel_face;` | — |
| `crates/slot-gfx/src/lib.rs` | `mod fit;` | — |
| `crates/slot-gfx/src/pipeline.rs` | `GamePass` field `pub(crate) soft: crate::fit::Soft`, set to `crate::fit::Soft::new(prog)`; the program is built from `&crate::fit::game_frag(GAME_FRAG)`; `self.soft.upload(tex == self.game);` after the `u_mode` uniform in `draw_source`: only the live game is softened, never a still (a screenshot) | `fit::game_frag`, `fit::Soft` |
| `crates/slot-gfx/src/fbo.rs` | `Compositor::set_soft_picture(on)` → `self.game.soft.set(on)`, before `set_game_source_rect` | — |
| `crates/slot/tests/app.rs` | Upstream's `l_and_r_change_the_mode_on_a_game_boy_cart_and_not_on_a_gba_one` and `the_picture_mode_is_remembered_per_cart`, and the `GB_WINDOW` constant only they used, are removed: they expect two modes. The fork's versions are in `fit_app.rs` | — |
| `site/docs/tour.md`, `site/docs/guide/controls.md`, `site/docs/guide/screen.md` | One line each describing L1/R1: L1 goes to full height first, then stretched; R1 steps back; bezels are hidden only while stretched | Take upstream's markup, re-apply the wording |

`crates/slot-gfx/src/shaders.rs` has no fork change: `fit::game_frag` patches upstream's shader
text when the program is built, at three anchors (`PATCHES`): after `varying vec2 v_uv;` it adds
`u_soft` and `soft_texel`, the first sample becomes
`u_soft > 0.5 ? soft_texel(uv) : texture2D(u_game, uv).rgb`, and the Grid branch's `hp`/`edge`
lines get `cells` (2 while softened, else 1). If an anchor is missing or not unique, slot logs it
and runs upstream's shader unpatched: every mode works as upstream's, and Fit only loses its
softening and finer grid. `fit::tests::the_fit_patches_apply_to_upstreams_game_shader` then fails
on any machine, so the sync catches it.

### Other fork changes to upstream files

| File | Fork change | Keep when resolving |
|---|---|---|
| `.github/workflows/release.yml` | Strips `-gp-custom*` before the `CHANGELOG.md` lookup | Re-applied by `sync_upstream.py` itself (section 5) |
| `.gitignore` | `/slot-v*/` and `/slot-v*-h700.zip` appended | Keep upstream's file, re-append the two lines |

**Rule: never reformat or lint-fix upstream's code.** Upstream does not run `cargo fmt` or clippy
in CI, and commits code they would change. The fork used to run `cargo fmt --all` and fix lints,
which rewrote lines all over `app.rs`, `frontend.rs` and `lib.rs`, the files upstream edits most,
and made every sync conflict there. Now only the fork's files are formatted and linted
(section 4), and upstream's files stay exactly as upstream wrote them, so they merge cleanly
unless upstream edits a hook's own lines.

### How to resolve a conflict

1. **Upstream wins.** Take upstream's version of the file and re-add the hooks from the table
   above. Never drop an upstream change to make a hook fit, and never move fork logic into an
   upstream file: adapt the `fit*.rs` file instead.
2. **Likely upstream changes and what to do:**
   - *Upstream adds its own picture mode or changes `VideoMode`:* keep their variant, keep `Fit`,
     and decide its place in `bigger()`/`smaller()` in `slot/src/fit.rs` (fit stays between
     actual-size and stretched). Every `match` on `VideoMode` must cover `Fit`. If upstream adds a
     mode that does what Fit does, drop the fork's version, keep `"fit"` parsing as an alias, and
     say so in the PR.
   - *Upstream changes the L1/R1 bindings or `slot_owns_the_shoulders`:* keep their guard and
     binding, swap only the target to `bigger()`/`smaller()`.
   - *Upstream adds a `BezelPick` variant:* nothing to do. `fit::game_bezel` hides every pick but
     `Default` in Fit, since they are drawn for the Actual picture.
   - *Upstream changes `GAME_FRAG`:* the shader merges cleanly; if the patch test fails, update
     `PATCHES` in `slot-gfx/src/fit.rs` to upstream's new text. `soft_texel` depends on `u_src`,
     `u_uv`, `u_rect` and `u_fbo` keeping their meaning.
   - *Upstream changes `GamePass` or how it builds the program:* re-add the `soft` field, its
     `Soft::new(prog)`, the `game_frag` wrap and the `upload()` call.
   - *Upstream changes the texture size* (`SRC_W`/`SRC_H`, now 240x160) *or the canvas*
     (`OUT_W`/`OUT_H`, now 720x480): the Fit rect is computed from them, but re-check the pixel
     numbers in the tests (black at x 91 and 628, picture at 95 and 624) and the bar width the
     bezel art assumes (x 0..93 and 627..720).
   - *Upstream edits one of the two tests the fork removed from `app.rs`:* keep them removed and
     carry upstream's new setup into the fork's versions in `fit_app.rs`.
3. **Assumption to keep true:** Fit's side bars are the game texture's own black margins. The real
   cores leave them black (`crates/slot/tests/render_picture_modes.rs` asserts it). The mock core
   (`crates/slot-retro/src/mock.rs`) fills the whole texture, so never test "pixel in the bar is
   black" against the mock.
4. **Commit after resolving:** the resolution goes in the merge commit on the sync branch; fork
   commits are never rewritten. The workflow adds the sync's record to `rebase-log.md` in that
   same commit. When fixing a sync pull request by hand, push to its `sync/upstream-<sha>`
   branch and update the record in the description.

## 4. Verify after every sync

```sh
python3 .github/sync/sync_upstream.py fork-fmt --check   # rustfmt, the fork's fit*.rs files only
python3 .github/sync/sync_upstream.py fork-clippy        # compile errors anywhere, lints in fit*.rs
python3 .github/sync/sync_upstream.py fork-clippy device
python3 .github/sync/sync_upstream.py fork-tests         # every Fit test by name
SLOT_SILENT=1 cargo test --workspace --no-fail-fast --lib --bins --tests
```

`fork-fmt` without `--check` formats the fork's files. **Do not run `cargo fmt --all` or upstream's
`task fmt`/`task check`**: they reformat upstream's code (section 3). `fork-clippy` counts lints in
upstream's code but does not fail on them. The test run leaves out examples because upstream's
`examples/record.rs` only builds on macOS. A local device build is
`task dist:device OUT=slot-<tag>` (see `RELEASE.md`).

### Fit tests: what each one guards

`python3 .github/sync/sync_upstream.py fork-tests` runs every test below by name and fails if any
of them is missing or failing; the sync workflow requires it. Its list, `FORK_TESTS` in that
script, is the source of truth: add a test there when you add one here. Tests marked (macOS) need
headless GL and are skipped on Linux, so run the command on a Mac after a sync that touched
rendering. File paths are under `crates/`.

| Feature | Tests |
|---|---|
| The fork's shader patch still applies to upstream's `GAME_FRAG` (any machine) | `slot-gfx/src/fit.rs` `fit::tests::the_fit_patches_apply_to_upstreams_game_shader` |
| A shader the patch does not fit is used exactly as upstream wrote it, never a crash (any machine) | `slot-gfx/src/fit.rs` `fit::tests::a_shader_the_patches_do_not_fit_is_left_as_upstream_wrote_it` |
| Screenshots are drawn as upstream draws them, even after a fitted game (macOS) | `slot-gfx/tests/fit_pipeline.rs` `a_screenshot_is_never_softened_even_after_a_fitted_game` |
| A fitted picture never reads the card bezel it hides from the card (macOS) | `slot/tests/fit_render_bezel.rs` `a_fitted_picture_never_reads_the_card_bezel_it_hides` |
| Fit between Actual and Stretch; L1 bigger, R1 smaller, stopping at both ends; GBA unaffected | `slot/tests/fit_app.rs` `l_and_r_change_the_mode_on_a_game_boy_cart_and_not_on_a_gba_one` |
| Fit crop: full height at 10:9, `[12/240, 8/160, 216/240, 144/160]` | the same test, and `the_picture_mode_is_remembered_per_cart` |
| `fit` saved per game and restored | `slot/tests/fit_app.rs` `the_picture_mode_is_remembered_per_cart` |
| A GBA game saved as `fit` shows its whole picture, unsoftened | `slot/tests/fit_app.rs` `a_gba_cart_saved_as_fit_still_shows_the_whole_picture` |
| Only Fit is softened (GB) | `slot/tests/fit_app.rs` `only_the_fitted_picture_is_softened` |
| GBC fits and softens exactly like GB, softened only while fitted | `slot/tests/fit_softening.rs` `a_game_boy_color_cart_is_softened_only_while_fitted`, `game_boy_and_game_boy_color_carts_fit_and_soften_alike` |
| Bezel kept in Fit, dropped in Stretch | `slot/tests/fit_app.rs` `the_fitted_picture_keeps_its_bezel_and_the_stretched_one_drops_it` |
| Fit bezel art (GB and GBC) fills the side bars and only shades the picture's edges, with the Actual frame's shadow | `slot-ui/tests/fit_bezel.rs` `the_fit_bezels_fill_the_side_bars_and_only_shade_the_picture_edges` |
| The fitted picture fills the height, black bars only at the sides (macOS) | `slot-gfx/tests/fit_pipeline.rs` `the_fitted_picture_fills_the_height_with_bars_only_at_the_sides` |
| Sharp-bilinear softening gives every row the same weight (macOS) | `slot-gfx/tests/fit_pipeline.rs` `softening_gives_every_fitted_game_boy_row_the_same_weight` |
| Grid in Fit: a line every half game pixel, evenly; unsoftened (Actual, Stretch) keeps one per game pixel (macOS) | `slot-gfx/tests/fit_pipeline.rs` `the_fitted_grid_lines_every_half_game_pixel_evenly` |
| Softening on colour stripes: every channel even, edges only mix the two colours (macOS) | `slot-gfx/tests/fit_colour.rs` `softened_colour_rows_stay_even_in_every_channel_without_fringes`, `softened_colour_columns_stay_even_in_every_channel_without_fringes` |
| Default bezel in Fit drawn on screen, power lamp and label (macOS, GBC) | `slot/tests/fit_render_bezel.rs` `the_default_bezel_in_fit_mode_lights_the_power_lamp_and_labels_the_right_bar` |
| A card bezel shows at Actual size and stays off the fitted picture (macOS) | `slot/tests/fit_render_bezel.rs` `a_bezel_from_the_card_shows_at_actual_size_and_stays_off_the_fitted_picture` |

Not covered yet: L1/R1 leaving the mode alone when the button is remapped to the game (upstream's
`bound_to_game` guard), `compose()` switching softening on during play (`set_soft_picture`), the
GB (non-colour) Fit bezel on screen, Fit → Actual bringing the normal bezel back, and None/AGS-102
showing no bezel in Fit.

**When merging:** the fork's tests are kept like the fork's code. Never delete, rename, `#[ignore]`
or loosen one, and never change what it asserts. If upstream changes an API a fork test uses
(session setup, `Compositor` methods, test helpers), update only the setup. If a fork test fails
after a merge, the merge is wrong: fix the code, not the test. All fork tests are in the fork's
own files; the only fork change to an upstream test file is the two tests removed from
`slot/tests/app.rs` (section 3).

- Renderer tests are `#![cfg(target_os = "macos")]` (headless GL) and **silently pass elsewhere**.
  A green run on Linux CI proves nothing about rendering; run on macOS.
- **Known failures not caused by the fork:** in `crates/slot-gfx/tests/layout.rs`,
  `a_framed_frame_on_a_4_by_3_panel_is_todays_frame` and
  `a_framed_frame_after_a_native_one_is_todays_frame_again` fail identically on unpatched `e5b12ad`
  on a Mac. They compare against stored frame hashes that probably differ per GPU. Do not "fix"
  them in the fork.

## 5. Releases

The sync workflow tags each merged upstream sync as the next `-gp-custom.<n>` release (section
1, step 6); other releases are tagged by hand. See `RELEASE.md`. In short: `release.yml` (upstream's) runs on any `v*` tag and
attaches `slot-<tag>-h700.zip`, which holds a `slot-<tag>/` folder for SD2. One zip serves every
H700 device.

`release.yml` has one fork change: it takes the release notes from the `CHANGELOG.md` section
named after the tag, and the fork strips `-gp-custom.<n>` from the tag first, so
`v1.9.0-gp-custom.1` gets upstream's `## [1.9.0]` notes. **Rule:** do not add fork sections to
`CHANGELOG.md` for this. The agent may not edit `.github/`, so `sync_upstream.py prepare`
re-applies the strip itself (`release_notes_for_fork`): on a conflict it takes upstream's file and
rewrites the `awk -v head=...` lookup. If upstream changes that lookup so it no longer matches,
the sync stops and asks for a hand fix on the sync branch. `.gitignore` ignores `/slot-v*/` and `/slot-v*-h700.zip` (two lines appended at the
end; if `.gitignore` conflicts, keep upstream's file and re-append them).

`pages.yml` is also upstream's. It deploys `site/` on pushes to `main` that touch it, and the repo
has upstream's `CNAME` (`slot-cfw.fyi`). In the fork it either fails or publishes a copy under that
domain setting; disable Pages or the workflow in the fork's settings rather than editing the file.

## 6. Open issues and decisions

- **Tag `v1.7.0-gp-custom` does not build:** it points to `16c72c7`, the first version of the fork
  commit, which compiles in `assets/bezel_gb_fit.png` and `assets/bezel_gbc_fit.png` with
  `include_bytes!` but did not contain them. The fork commit on `main` (`0b46d92`) was amended to
  add them. Every later tag builds; do not reuse or move the broken one.
- **Tags so far:** `v1.7.0-gp-custom.1`–`.4`, `v1.8.0-gp-custom.1`, `v1.8.1-gp-custom.1`,
  `v1.9.0-gp-custom.1` and `.2`. `v1.9.0-gp-custom.1` (`ed4518d`) and `.2` (`8575439`) were tagged
  by hand on fork-only changes, not by a sync, so the next sync after upstream's next commit on
  `v1.9.0` gives `v1.9.0-gp-custom.3`.
- **Not verified on hardware:** the device build has not been run on a real RG SP with a real
  game, in particular scrolling with softening on and the shader on the device's GLSL ES path.
- **4:3 panels** (640x480: RG35XXSP, RG35XX Plus, RG28XX, RG35XX-H): slot scales its 720x480
  canvas to 640x427 there, so Fit fills the canvas height, not the panel's, and thin top and bottom
  bars remain. Fixing it means drawing outside the canvas. Out of scope.
- **Planned, not built:** per-mode bezel folders on the card, `Bezels/<GB|GBC>/actual/` and
  `Bezels/<GB|GBC>/fit/` (PNGs directly in `Bezels/<GB|GBC>/` stay Actual bezels). Undecided:
  whether Fit uses the `fit/` file with the same name as the Actual pick, or gets its own pick (a
  new in-game menu row saved as e.g. `bezel_gbc_fit=` in `Config/slot.state`). Until then card
  bezels are hidden in Fit.
- **Trademark:** the Fit bezel label approximates Nintendo's "GAME BOY COLOR" logo in Arial Bold
  Italic. Keep that in mind before publishing the art more widely.

## 7. Background on slot that the change depends on

- **Picture pipeline:** the core renders into a 240x160 texture (`SRC_W` x `SRC_H`, canvas / 3).
  A GB frame (160x144) sits centred at x 40..200, y 8..152 with black margins. `source_rect`
  returns `[x, y, w, h]` in 0..1 texture units, drawn over the whole 720x480 canvas
  (`OUT_W` x `OUT_H`).
- **Bezels:** picked per console (GB, GBC, GBA), not per game, from the in-game menu (MENU →
  Bezel, Left/Right cycles None → Default → AGS-102 (4:3 only) → files in `Bezels/<GB|GBC>/`).
  Saved in `Config/slot.state` as `bezel_gb=`, `bezel_gbc=`, `bezel_gba=` with `none`,
  `default`, `ags-102` or a file name; a missing file resolves to None (`App::resolved_bezel`).
  A bezel is a 720x480 RGBA image over the canvas; the built-in Actual window is about
  x 126..593, y 32..451. On a 720x480 panel the saved default is `ags-102`, which is not offered
  there, so bezels are off until the user picks Default.
- **Fit bezel art:** both PNGs are 720x480 RGBA and copy the Actual bezel's screen surround
  into the side bars, so the picture looks set in the same screen. Each bar, from the outside in:
  the Actual bezel's grey body (x 0..82 and 638..720), its 2 px highlight rim, its 9 px black
  frame (x 84..93 and 627..636), then the same 7 px inner shadow the Actual frame casts over the
  picture (black, alpha 135 at the picture's edge fading to 8, x 93..100 and 620..627). The rest
  of the picture is transparent. The lamp and its BATTERY/POWER label are cut from the Actual
  bezel at the same height and centred in the left bar; the right bar carries GAME BOY (COLOR),
  running bottom to top, centred at x 679 on the middle row, in the Actual label's grey.
  `python3 tools/fit_bezel/make_fit_bezels.py` (needs ffmpeg) builds both from
  `bezel_gb(c).png`. If upstream redraws those, re-run it and check the frame and rim columns
  it reads (the constants at its top) still match.
