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
[slot](https://github.com/BrandonKowalski/slot) with one addition: a **Fit** picture mode for
Game Boy and Game Boy Color games.

## Fit mode for GB and GBC

slot shows GB and GBC games in two modes: **Actual** (exact 3x, with the bezel) and **Stretch**
(full width, no bezel).

> [!IMPORTANT]
> **The fork adds Fit between them.** Fit fills the whole screen height and keeps the Game Boy's
> own 10:9 shape, so the picture is about 533x480 with a bar on each side.

- **L1** steps bigger: Actual → Fit → Stretch.
- **R1** steps smaller: Stretch → Fit → Actual.
- The mode is saved per game, like slot's other picture modes.
- The side bars get a Fit version of the built-in Game Boy and Game Boy Color bezels.
- Fit is softened with a sharp-bilinear filter. At 3.33x, plain nearest-neighbour scaling makes
  rows alternate between 3 and 4 pixels tall, which shows as shimmer when the game scrolls.
- GBA games have no Fit mode and are not affected.

### Fit with bezels

The side bars hold Fit versions of the built-in bezels: Game Boy (left) and Game Boy Color (right).

<p align="center">
<img src=".github/resources/fork/fit-gb.png" width="48%" alt="Fit mode on a Game Boy game, with the Fit bezel's battery lamp and GAME BOY label in the side bars">
<img src=".github/resources/fork/fit-gbc.png" width="48%" alt="Fit mode on a Game Boy Color game, with the Fit bezel's power lamp and GAME BOY COLOR label in the side bars">
</p>

### Fit softened

In each picture, the left side is hard pixels at 3.33x and the right side is the softened Fit
picture: every row ends up the same height.

<p align="center">
<img src=".github/resources/fork/fit-softening.png" width="96.5%" alt="Left: hard pixels at 3.33x with uneven rows. Right: the softened Fit picture with even rows">
</p>

<p align="center">
<img src=".github/resources/fork/fit-softening-zoom.png" width="60%" alt="Close-up of the same comparison: uneven rows on the left, even rows on the right">
</p>

### Screen sizes

Fit is designed for 3:2 panels such as the RG SP (720x480), where the side bars are about 93 px
wide. On 4:3 panels it still works, but thin bars stay at the top and bottom.

## Always on the newest slot

The fork keeps up with upstream on its own. Once a day, a workflow merges any new slot changes
into the fork, re-applies Fit on top, runs the build and the fork's tests, and publishes a new
fork release, so the fork stays on the newest slot version.

Fork releases are tagged `v<slot version>-gp-custom.<n>`, for example `v1.9.0-gp-custom.1` is
slot 1.9.0 plus Fit. Their release notes are slot's.

> [!IMPORTANT]
> **Download the latest fork release from the
> [releases page](https://github.com/Maxcimkj/slot-gb-custom/releases/latest).** Install it the
> same way as slot.

## Full slot documentation

For everything else (installation, controls, cores, bezels, link play, settings), see the
[slot README](https://github.com/BrandonKowalski/slot/blob/main/README.md) and the
user guide at [slot-cfw.fyi](https://slot-cfw.fyi).
