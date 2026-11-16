# Fork notes

Context for whoever syncs this fork with upstream, human or AI agent, without the conversations
that produced it. Read this before resolving a conflict.

## 1. What this fork is

- **Fork:** `Maxcimkj/slot-gb-custom`, default branch `main`.
- **Upstream:** `BrandonKowalski/slot`, branch `main`. slot is a Rust frontend for Anbernic H700
  handhelds (RG SP, RG34XX, RG35XX family, RG28XX, RG40XX, CubeXX) that plays GB, GBC and GBA
  games through the mGBA and gpSP libretro cores.
- **Purpose:** keep everything from upstream and add one GB/GBC shader upstream does not have,
  **Shimmerless Grid**: upstream's Shimmerless smoothing and upstream's Grid lines in one pass. Publish
  device zips from the fork. The target device is the RG SP (720x480 panel), with Settings →
  Screen → Game Boy → Scale set to upstream's Fit.
- **History:** the fork started (after upstream `v1.7.0`) by adding its own Fit picture mode,
  sharp-bilinear softening and Fit bezel art. Upstream later shipped all of that itself: the Fit
  scale (`103aa7d`), Fit bezels (`d2edb86`), per-Scale card bezels (`5903cf4`) and the
  Shimmerless shader (`ded7ba2`), which is the same smoothing. The fork then dropped its Fit
  (branch `smooth-grid-filter`) and kept only what upstream lacks: smoothing and grid together.
  `rebase-log.md` has every sync. Fork releases are tagged `v<upstream version>-gp-custom.<n>`.
- **Sync model:** upstream `main` is merged into the fork's `main` through a pull request, the
  way GitHub's Sync fork → Open pull request does it; history is never rewritten. A coding agent
  resolves the pull request, and the workflow merges it once every required check passes.
  `.github/workflows/sync-upstream.yml` does this weekly (Mondays 05:17 UTC) and on demand:
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
     build (host and device), the device build and the fork's Shimmerless Grid tests by name (section 4) are
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
| `RELEASE.md` | How release zips are built and named. Kept out of `README.md` on purpose so syncs never touch it |
| `rebase-log.md` | One record per automatic upstream sync, newest first, written by the sync workflow |
| `.github/workflows/sync-upstream.yml`, `.github/sync/sync_upstream.py` | The automatic upstream sync pull request, resolved by the OpenCode agent (section 1) |
| `crates/slot-gfx/tests/fit_shimmerless_grid_shader.rs` | Reads `src/shaders.rs` as text: the Shimmerless Grid branch is still first in the chain, at `ScreenEffect::ShimmerlessGrid`'s number (any machine) |
| `crates/slot-gfx/tests/fit_shimmerless_grid.rs` | Shimmerless Grid through the real shader, against fixed expectations (no other filter is rendered): full height in Fit, even lines and rows, sharp blends only, no blending at a whole scale (macOS only) |
| `crates/slot/tests/fit_shimmerless_grid_menu.rs` | Shimmerless Grid in the GB/GBC Shader rows and the shader shortcut, saved and read back, never offered for GBA |

**Rule: every fork Rust file is named `fit*.rs`** (`crates/*/src/fit*.rs`, `crates/*/tests/fit*.rs`).
The sync formats and lints exactly those files (`FORK_RUST` in `sync_upstream.py`), so a new fork
file outside that pattern would go unchecked. The fork's code in upstream's files is only the hooks
of section 3, including the Shimmerless Grid branch in `shaders.rs`; never add more there.

**Rule:** `README.md` belongs to the fork. Upstream's changes to it are never taken: the sync
keeps the fork's version and records that in `rebase-log.md`. It keeps upstream's top part (banner,
What is slot?, What does it look like?, Supported Devices) plus the fork's own section, and points
to upstream's README for the full documentation. If upstream's top part changes in a way that
matters (a new device, new screenshots), copy it over by hand. Do not edit `CHANGELOG.md` or
`site/` in the fork: Shimmerless Grid is documented in `README.md` only. If upstream adds a file with
one of the other names above, rename ours, not theirs.

## 3. Fork changes to upstream files (conflict hotspots)

Upstream files carry only the hooks below and are otherwise byte for byte upstream's, formatting
included. `git diff upstream/main main -- crates/` shows exactly these plus the `fit*.rs` test files.

### The feature in one paragraph

Upstream's Fit scale shows a Game Boy picture at about 3.33x (533x480 on the RG SP). Its Grid
shader draws a line on every game pixel's edge but samples nearest-neighbour, so rows alternate
between 3 and 4 screen lines; its Shimmerless shader blends only across each edge, so every row
is even, but draws no lines. **Shimmerless Grid** runs Shimmerless's sampling and then Grid's darkening
in one branch of upstream's game shader. At a whole scale Shimmerless changes nothing, so Shimmerless
Grid there is exactly Grid. It is offered for GB and GBC (after Shimmerless) and saved as
`shimmerless-grid` in `shader_gb=` / `shader_gbc=`; GBA never offers it, and a GBA `shimmerless-grid`
falls back to upstream's default like any unknown value.

### The hooks in upstream files

Every hook is written the way upstream wrote `Shimmerless` next to it. When an upstream file
conflicts, take upstream's version and re-add its hooks.

| File | Hook |
|---|---|
| `crates/slot-store/src/slot_state.rs` | `ShimmerlessGrid` last in `enum Shader`; last in `Shader::GB` (array length + 1); `Shader::ShimmerlessGrid => "shimmerless-grid"` in `name()`; last in `parse()`'s list. `Shader::GBA` unchanged |
| `crates/slot-gfx/src/lcd3x.rs` | `ShimmerlessGrid` last in `enum ScreenEffect` (its `mode()` is its index, 6, which the shader branch below tests for) |
| `crates/slot-gfx/src/shaders.rs` | In `GAME_FRAG`, the first branch after `if (!inside) {\n    }`: `} else if (abs(u_mode - 6.0) < 0.5) {` (exactly mode 6, so it can never catch a mode upstream adds) holding its own copy of the Shimmerless code (as in the `u_mode > 4.5` branch) followed by its own copy of the Grid code (as in the `u_mode > 1.5` branch), sharing Shimmerless's `hp` |
| `crates/slot/src/frontend.rs` | in `compose()`'s shader `match`: `slot_store::Shader::ShimmerlessGrid => ScreenEffect::ShimmerlessGrid,` |
| `crates/slot/src/app.rs` | the shader toast `match`: `Shader::ShimmerlessGrid => Toast::ShaderShimmerlessGrid,`; the quick menu value `match`: `Shader::ShimmerlessGrid => QuickValue::ShimmerlessGrid,` |
| `crates/slot-ui/src/toast.rs` | `ShaderShimmerlessGrid` after `ShaderShimmerless` in the enum and in `FIXED` (array length + 1); text `"Shader: Shimmerless Grid"`; in `setting()`, `| Toast::ShaderShimmerlessGrid` added to the wide toasts (`NeedsMgba | NoWifi | PeerEnded`), since the text does not fit the normal 240 px toast at full size |
| `crates/slot-ui/src/quick_menu.rs` | `ShimmerlessGrid` after `Shimmerless` in `enum QuickValue` and in `ALL` (array length + 1); text `"Shimmerless Grid"` |
| `crates/slot-ui/tests/quick_menu.rs`, `crates/slot-ui/tests/toast.rs` | `"Shimmerless Grid"` / `Toast::ShaderShimmerlessGrid` after Shimmerless's entry in the lists these upstream tests spell out in full; in `toast.rs`'s `the_banner_says_what_happened_and_never_what_is_on_screen` the fixed-banner count (`all[..N]`, `all[N..N + palettes]`, `all[N + palettes..]`) is upstream's plus one |
| `crates/slot-gfx/tests/pipeline.rs`, `crates/slot-gfx/tests/quad.rs`, `crates/slot/tests/render_fetch_art.rs`, `crates/slot/tests/render_studio.rs` | `#![cfg(target_os = "macos")]` at the top: they use `HeadlessSurface`, which only exists on macOS, and the sync runner is Linux. Add the same line to any new upstream test that uses it |

Shimmerless Grid is written into upstream's shader like the other filters, so it reads like
them and the program is built as upstream builds it. It has its own copy of the Shimmerless and
Grid code, so it does not depend on those modes: when upstream changes Shimmerless or Grid,
Shimmerless Grid stays as it is. Update it by hand only if you want an upstream improvement in it too.

### Other fork changes to upstream files

| File | Fork change | Keep when resolving |
|---|---|---|
| `.github/workflows/release.yml` | Strips `-gp-custom*` before the `CHANGELOG.md` lookup | Re-applied by `sync_upstream.py` itself (section 5) |
| `.gitignore` | `/slot-v*/` and `/slot-v*-h700.zip` appended | Keep upstream's file, re-append the two lines |

**Rule: never reformat or lint-fix upstream's code.** Upstream does not run `cargo fmt` or clippy
in CI, and commits code they would change. Only the fork's files are formatted and linted
(section 4), and upstream's files stay exactly as upstream wrote them, so they merge cleanly
unless upstream edits a hook's own lines.

### How to resolve a conflict

1. **Upstream wins.** Take upstream's version of the file and re-add the hooks from the table
   above. Never drop an upstream change to make a hook fit, and never move fork logic into an
   upstream file beyond the hooks above.
2. **Likely upstream changes and what to do:**
   - *Upstream adds a shader* (a new `Shader`, `ScreenEffect`, `Toast`, `QuickValue` variant, or a
     longer `Shader::GB`): keep theirs where they put it and put `ShimmerlessGrid` after it, last, in
     every list and enum; bump each array length by one more. Upstream's new branch will usually be
     `} else if (u_mode > 5.5) {` at the top of `GAME_FRAG`, right where ours starts: keep theirs
     below ours and change ours to the new index (`abs(u_mode - 7.0) < 0.5` for index 7);
     `shimmerless_grid_is_the_first_branch_and_matches_only_its_effect` reads the index from
     `ScreenEffect::ShimmerlessGrid` and checks it. If a fork menu test
     fails because the new shader now sits between Shimmerless and Shimmerless Grid, update only its
     setup (the step sequence), not what it checks.
   - *Upstream changes Shimmerless or Grid in `GAME_FRAG`:* take upstream's change to their
     branches and leave the Shimmerless Grid branch as it is; it is independent of them.
   - *Upstream adds a Shimmerless Grid of its own* (smoothing and grid together): drop the fork's, keep
     `"shimmerless-grid"` parsing as an alias for theirs, and say so in the PR.
3. **Commit after resolving:** the resolution goes in the merge commit on the sync branch; fork
   commits are never rewritten. The workflow adds the sync's record to `rebase-log.md` in that
   same commit. When fixing a sync pull request by hand, push to its `sync/upstream-<sha>`
   branch and update the record in the description.

## 4. Verify after every sync

```sh
python3 .github/sync/sync_upstream.py fork-fmt --check   # rustfmt, the fork's fit*.rs files only
python3 .github/sync/sync_upstream.py fork-clippy        # compile errors anywhere, lints in fit*.rs
python3 .github/sync/sync_upstream.py fork-clippy device
python3 .github/sync/sync_upstream.py fork-tests         # every Shimmerless Grid test by name
SLOT_SILENT=1 cargo test --workspace --no-fail-fast --lib --bins --tests
```

`fork-fmt` without `--check` formats the fork's files. **Do not run `cargo fmt --all` or upstream's
`task fmt`/`task check`**: they reformat upstream's code (section 3). `fork-clippy` counts lints in
upstream's code but does not fail on them. The test run leaves out examples because upstream's
`examples/record.rs` only builds on macOS. A local device build is
`task dist:device OUT=slot-<tag>` (see `RELEASE.md`).

### Shimmerless Grid tests: what each one guards

`python3 .github/sync/sync_upstream.py fork-tests` runs every test below by name and fails if any
of them is missing or failing; the sync workflow requires it. Its list, `FORK_TESTS` in that
script, is the source of truth: add a test there when you add one here. Tests marked (macOS) need
headless GL and are skipped on Linux, so run the command on a Mac after a sync that touched
rendering. File paths are under `crates/`.

| Feature | Tests |
|---|---|
| It is the first branch and matches exactly `ScreenEffect::ShimmerlessGrid`'s number; whitespace is ignored (any machine) | `slot-gfx/tests/fit_shimmerless_grid_shader.rs` `shimmerless_grid_is_the_first_branch_and_matches_only_its_effect` |
| In Fit, every pixel (grid darkening taken out) is a blend of only the game pixels it touches, and most are one game pixel's own colour: no fringes, not blurred (macOS) | `slot-gfx/tests/fit_shimmerless_grid.rs` `shimmerless_grid_in_fit_blends_only_neighbouring_game_pixels_and_stays_sharp` |
| The fitted picture fills the height, black bars only at the sides (macOS) | `slot-gfx/tests/fit_shimmerless_grid.rs` `the_fitted_picture_fills_the_height_with_bars_only_at_the_sides` |
| One even line per game pixel, on its edge and none inside it, on 3:2 and 4:3 panels (macOS) | `slot-gfx/tests/fit_shimmerless_grid.rs` `the_fitted_shimmerless_grid_lines_every_game_pixel_evenly` |
| Every fitted row weighs the same, 480 / 144 = 3.33 screen lines (macOS) | `slot-gfx/tests/fit_shimmerless_grid.rs` `shimmerless_grid_gives_every_fitted_row_the_same_weight` |
| At Integer (3x) nothing is blended: every pixel is its own game pixel's colour under the grid (macOS) | `slot-gfx/tests/fit_shimmerless_grid.rs` `shimmerless_grid_at_a_whole_scale_does_not_blend` |
| GB and GBC Shader rows: Shimmerless → Shimmerless Grid → Off, saved each step | `slot/tests/fit_shimmerless_grid_menu.rs` `the_game_boy_shader_rows_step_to_shimmerless_grid_after_shimmerless_and_wrap_to_off` |
| GBA never offers it; a GBA `shimmerless-grid` falls back to the default | `slot/tests/fit_shimmerless_grid_menu.rs` `the_gba_shader_row_never_offers_shimmerless_grid`, `a_gba_shader_saved_as_shimmerless_grid_falls_back_to_the_default` |
| Saved as `shimmerless-grid` and read back | `slot/tests/fit_shimmerless_grid_menu.rs` `shimmerless_grid_is_saved_by_name_and_read_back` |
| The in-game shader shortcut reaches it | `slot/tests/fit_shimmerless_grid_menu.rs` `the_shader_shortcut_reaches_shimmerless_grid_on_a_game_boy_color_game` |

**When merging:** the fork's tests are kept like the fork's code. Never delete, rename, `#[ignore]`
or loosen one, and never change what it asserts. If upstream changes an API a fork test uses
(session setup, `Compositor` methods, test helpers), update only the setup. If a fork test fails
after a merge, the merge is wrong: fix the code, not the test.

- Renderer tests are `#![cfg(target_os = "macos")]` (headless GL) and **silently pass elsewhere**.
  A green run on Linux CI proves nothing about rendering; run on macOS.
- `slot-ui` `toast::tests::every_toast_is_set_at_full_size` checks every toast's text fits its box
  at full size. A fork toast that does not fit goes in the wide-toast list (section 3), as upstream
  does for its own long toasts.

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

- **Not verified on hardware:** Shimmerless Grid has not been run on a real RG SP with a real game,
  in particular on the device's GLSL ES path.
- **Same picture as the old fork Fit + Grid, not the same pixels:** the old fork drew Fit at x
  93.33..626.67 (533.33 px); upstream rounds it to x 93..626 (533 px). Smoothing and grid are the
  same, overall brightness and row/column evenness match, but the lines sit about a third of a
  pixel apart, so frames differ pixel by pixel.
- **Settings left from the old Fit:** `fit` entries in `Config/video_mode.txt` no longer parse
  and fall back to Actual; users set Scale → Fit once per console instead.
- **Tags so far:** the old fork Fit is in `v1.7.0-gp-custom` to `v1.12.0-gp-custom.3` and in
  `v1.13.0-gp-custom.1` (a sync that ran before Shimmerless Grid reached `main`). Shimmerless Grid
  is in `v1.12.0-gp-custom.4` and from `v1.13.0-gp-custom.2` on. Old tags stay as they are;
  `v1.7.0-gp-custom` does not build (its Fit bezel art was missing) and must not be reused.

## 7. Background on slot that the change depends on

- **Picture pipeline:** the core renders into a 240x160 texture (`SRC_W` x `SRC_H`). A GB frame
  (160x144) sits centred at x 40..200, y 8..152 with black margins. `GameLayout` places the
  picture on the panel per Scale (`Sizing::Fit` in `slot-gfx/src/placement.rs`), and the game
  shader gets the source rect (`u_uv`), the drawn rect (`u_rect`, `u_fbo`) and the picture's own
  rect (`u_pic`). Shimmerless and Grid both derive the scale from these (`hp` = half a screen
  pixel in texels), which is what Shimmerless Grid relies on.
- **Shaders:** chosen per console (`shader_gb`, `shader_gbc`, `shader_gba` in `Config/slot.state`)
  from Settings → Screen or the in-game menu, and stepped in game with SELECT + A / SELECT + B.
  The choice reaches the shader as `u_mode`, the `ScreenEffect`'s index.
