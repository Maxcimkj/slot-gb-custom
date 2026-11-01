<div align="center">

<img src=".github/resources/banner.png" width="100%" alt="slot. A bespoke, Game Boy-centric frontend for the Anbernic RG SP.">

<a href="LICENSE"><img src="https://img.shields.io/github/license/BrandonKowalski/slot?style=for-the-badge&labelColor=242429&color=4d4d57" height="36" alt="License"></a>
<a href="https://github.com/BrandonKowalski/slot/stargazers"><img src="https://img.shields.io/github/stars/BrandonKowalski/slot?style=for-the-badge&labelColor=242429&color=4d4d57" height="36" alt="Stars"></a>
<a href="https://github.com/BrandonKowalski/slot/releases"><img src="https://img.shields.io/github/downloads/BrandonKowalski/slot/total?style=for-the-badge&labelColor=242429&color=4d4d57" height="36" alt="Downloads"></a>
<a href="https://slot-cfw.fyi"><img src="https://img.shields.io/badge/Guide-slot--cfw.fyi-4d4d57?style=for-the-badge&labelColor=242429" height="36" alt="Guide"></a>
<a href="CHANGELOG.md"><img src="https://img.shields.io/badge/Changelog-4d4d57?style=for-the-badge&labelColor=242429" height="36" alt="Changelog"></a>

</div>

---

# What is slot?

A bespoke, Game Boy-centric frontend for the Anbernic RG SP.

Has support for GBA, GBC, and GB titles only.

> [!IMPORTANT]
> ROMs must be unzipped. slot does not read `.zip` or `.7z` files.

A full user guide can be found at [slot-cfw.fyi](https://slot-cfw.fyi).

Release notes can be found in the [changelog](CHANGELOG.md).

---

# What does it look like?

<p align="center">
<img src=".github/resources/screenshots/carousel.gif" width="48%" alt="Scrolling the cart carousel across the Game Boy Advance, Game Boy and Game Boy Color shelves">
<img src=".github/resources/screenshots/core-picker.gif" width="48%" alt="Flipping a cart to its circuit board to choose mGBA or gpSP">
</p>

<p align="center">
<img src=".github/resources/screenshots/link.gif" width="96.5%" alt="Two SPs racing in Mario Kart over the emulated link">
</p>

---

# Supported Devices

| Device             | Supported                                    | Since Version |
|--------------------|----------------------------------------------|---------------|
| Anbernic RG SP     | Yes                                          | Always        |
| Anbernic RG34XX    | Yes                                          | 1.4.0         |
| Anbernic RG34XXSP  | Yes                                          | 1.4.0         |
| Anbernic RG35XXSP  | Yes                                          | 1.5.0         |
| Anbernic 35XX Plus | Yes                                          | 1.5.0         |
| Anbernic RG28XX    | Yes                                          | 1.6.0         |
| Anbernic RG35XX-H  | Yes                                          | 1.7.0         |

If you have one of the untested devices, or have tried slot on a device not listed,
please [open an issue on GitHub](https://github.com/BrandonKowalski/slot/issues) and let me know how it went.

---

# What does this fork bring?

This is [slot-gb-custom](https://github.com/Maxcimkj/slot-gb-custom), a fork of
[slot](https://github.com/BrandonKowalski/slot) with one addition: a **Shimmerless Grid** shader for
Game Boy and Game Boy Color games.

## Shimmerless Grid

slot's **Fit** scale (Settings → Screen → Game Boy → Scale) fills the screen height at the Game
Boy's own 10:9 shape: about 533x480 on the RG SP, 3.33 screen pixels per game pixel. At that
size slot offers two shaders that each fix half of the picture:

- **Shimmerless** blends only where two game pixels meet, so every row and column is the same
  size and nothing shimmers when the game scrolls. But the pixels are not outlined.
- **Grid** outlines every game pixel. But plain 3.33x scaling makes rows alternate between 3 and
  4 screen pixels.

> [!IMPORTANT]
> **The fork adds Shimmerless Grid, both at once:** Shimmerless's even pixels with Grid's lines over
> them. Pick it under Settings → Screen → Game Boy (or Game Boy Color) → Shader, or step to it
> in game with <kbd>SELECT</kbd> + <kbd>A</kbd> / <kbd>SELECT</kbd> + <kbd>B</kbd>.

- It comes after Shimmerless in the Game Boy and Game Boy Color shader lists and is saved like
  any other shader (`shader_gb=shimmerless-grid` in `Config/slot.state`).
- At a whole scale (Integer, SP Screen) there is nothing to smooth, and it looks exactly like Grid.
- GBA games do not offer it: their picture is always a whole 3x.
- Everything else is slot as it is, including the other shaders, Fit, its bezels and screenshots.

> [!NOTE]
> **Upgrading from a fork release before Shimmerless Grid** (`v1.12.0-gp-custom.3` or older)? The
> fork's own per-game Fit mode is gone: slot's Fit scale replaces it. Games you had switched to
> Fit with L1 now open at Actual size. Set Settings → Screen → Game Boy (and Game Boy Color) →
> Scale to **Fit** once, and every game of that console is fitted. L1/R1 still switch a game
> between that and Stretch.

## Always on the newest slot

The fork keeps up with upstream on its own. Once a week, a workflow merges any new slot changes
into the fork, re-applies Shimmerless Grid on top, runs the build and the fork's tests, and publishes
a new fork release, so the fork stays on the newest slot version.

To keep those merges easy, Shimmerless Grid is one more branch in slot's shader, written next to
Shimmerless and Grid with its own copy of their code, so changes slot makes to those two never
change it. slot's other files stay exactly as slot wrote them, apart from one-line hooks that add
the new choice next to Shimmerless. [`FORK-NOTES.md`](FORK-NOTES.md) lists every hook.

Fork releases are tagged `v<slot version>-gp-custom.<n>`, for example `v1.12.0-gp-custom.4` is
slot 1.12.0 plus Shimmerless Grid. Their release notes are slot's.

> [!IMPORTANT]
> **Download the latest fork release from the
> [releases page](https://github.com/Maxcimkj/slot-gb-custom/releases/latest).** Install it the
> same way as slot.

## Full slot documentation

For everything else (installation, controls, cores, bezels, link play, settings), see the
[slot README](https://github.com/BrandonKowalski/slot/blob/main/README.md) and the
user guide at [slot-cfw.fyi](https://slot-cfw.fyi).
