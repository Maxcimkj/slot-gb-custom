"""Build assets/bezel_gb_fit.png and assets/bezel_gbc_fit.png from the Actual bezels.

The fitted Game Boy picture spans x 93.3..626.7 of the 720x480 canvas at full height, so the
Fit bezel lives in the two side bars. Each bar is the Actual bezel cut down to fit: its grey
body, the 2 px highlight rim, the 9 px black screen frame, and the same 7 px inner shadow the
Actual frame casts over the picture's edge. The lamp and its label are cut from the Actual
bezel at the same height and centred in the left bar; the right bar carries the console's
name, recoloured to the Actual label grey.

Needs ffmpeg. Run from anywhere: python3 tools/fit_bezel/make_fit_bezels.py
"""

from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent

W, H = 720, 480
# Picture edges at x 93.3 and 626.7: frame up to 93, shadow from there inwards.
LEFT_FRAME = range(84, 93)
RIGHT_FRAME = range(627, 636)
LEFT_RIM = (82, 83)
RIGHT_RIM = (636, 637)
LEFT_BODY = range(0, 82)
RIGHT_BODY = range(638, W)
# The Actual bezel's frame: rim at x 109..110 and 609..610, black 111..119 and 600..608, and its
# inner shadow, strongest at the picture's edge.
ACTUAL_LEFT_RIM = (109, 110)
ACTUAL_RIGHT_RIM = (609, 610)
SHADOW = (135, 127, 110, 84, 51, 25, 8)
# Rows of the Actual frame's straight sides, away from its corners.
SIDE_ROWS = range(40, 440)

CONSOLES = {
    # Actual bezel, label sprite, x where the lamp art starts and ends, label grey.
    "gb": ("bezel_gb.png", "label_gb.png", (17, 92), (158, 153, 153)),
    "gbc": ("bezel_gbc.png", "label_gbc.png", (16, 87), (165, 143, 143)),
}
# The current label sprites are drawn in this grey.
LABEL_GREY = 207


def read(path: Path) -> tuple[bytes, int, int]:
    probe = subprocess.run(
        ["ffprobe", "-v", "error", "-select_streams", "v:0", "-show_entries",
         "stream=width,height", "-of", "csv=p=0", str(path)],
        check=True, capture_output=True, text=True,
    ).stdout.strip().split(",")
    raw = subprocess.run(
        ["ffmpeg", "-v", "error", "-i", str(path), "-f", "rawvideo", "-pix_fmt", "rgba", "-"],
        check=True, capture_output=True,
    ).stdout
    return raw, int(probe[0]), int(probe[1])


def write(path: Path, rgba: bytearray) -> None:
    subprocess.run(
        ["ffmpeg", "-v", "error", "-y", "-f", "rawvideo", "-pix_fmt", "rgba", "-s", f"{W}x{H}",
         "-i", "-", "-pix_fmt", "rgba", str(path)],
        check=True, input=bytes(rgba),
    )


def side_row(y: int) -> int:
    """Map any canvas row onto the Actual frame's straight sides, mirroring at their ends."""
    n = len(SIDE_ROWS)
    p = y % (2 * n)
    return SIDE_ROWS.start + min(p, 2 * n - 1 - p)


def build(actual: bytes, label: tuple[bytes, int, int], art: tuple[int, int], grey) -> bytearray:
    out = bytearray(W * H * 4)

    def put(x, y, px):
        out[(y * W + x) * 4:(y * W + x) * 4 + 4] = bytes(px)

    def src(x, y):
        return actual[(y * W + x) * 4:(y * W + x) * 4 + 4]

    # Left body: the Actual left body, shifted so the lamp art sits centred in the bar.
    shift = (art[0] + art[1] + 1) // 2 - (LEFT_BODY.stop // 2)
    # Right body: the Actual right body is plain grey from x 611, the same columns serve.
    for y in range(H):
        for x in LEFT_BODY:
            put(x, y, src(x + shift, y))
        for x in RIGHT_BODY:
            put(x, y, src(x, y))
        for x, ax in zip(LEFT_RIM, ACTUAL_LEFT_RIM):
            put(x, y, src(ax, side_row(y)))
        for x, ax in zip(RIGHT_RIM, ACTUAL_RIGHT_RIM):
            put(x, y, src(ax, side_row(y)))
        for x in [*LEFT_FRAME, *RIGHT_FRAME]:
            put(x, y, (0, 0, 0, 255))
        for d, a in enumerate(SHADOW):
            put(LEFT_FRAME.stop + d, y, (0, 0, 0, a))
            put(RIGHT_FRAME.start - 1 - d, y, (0, 0, 0, a))

    # The console's name, centred in the right body and on the screen's middle row.
    lab, lw, lh = label
    x0 = (RIGHT_BODY.start + RIGHT_BODY.stop - lw) // 2
    y0 = (H - lh) // 2
    for ly in range(lh):
        for lx in range(lw):
            r, g, b, a = lab[(ly * lw + lx) * 4:(ly * lw + lx) * 4 + 4]
            if a == 0:
                continue
            if r == g == b:
                r, g, b = (c * r // LABEL_GREY for c in grey)
            o = ((y0 + ly) * W + x0 + lx) * 4
            for ch, c in enumerate((r, g, b)):
                out[o + ch] = (c * a + out[o + ch] * (255 - a)) // 255
    return out


def main() -> None:
    for name, (bezel, sprite, art, grey) in CONSOLES.items():
        actual, w, h = read(ROOT / "assets" / bezel)
        assert (w, h) == (W, H), f"{bezel} is {w}x{h}"
        out = build(actual, read(HERE / sprite), art, grey)
        write(ROOT / "assets" / f"bezel_{name}_fit.png", out)
        print(f"wrote assets/bezel_{name}_fit.png")


if __name__ == "__main__":
    main()
