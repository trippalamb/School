import matplotlib.pyplot as plt

_segments = []


def add_line_segment(length, label, color="blue"):
    """Add a line segment to draw."""
    _segments.append({"length": length, "label": label, "color": color})


def draw_line_segments():
    """Draw all the line segments that have been added."""
    if not _segments:
        print("No segments to draw! Use add_line_segment() first.")
        return

    max_length = max(seg["length"] for seg in _segments)
    padding = 1
    width = max_length + 2 * padding
    row_height = 1.5
    height = len(_segments) * row_height + padding

    fig, ax = plt.subplots(figsize=(8, max(3, height)))

    for i, seg in enumerate(_segments):
        x1 = padding
        x2 = padding + seg["length"]
        y = height - padding - i * row_height

        ax.plot([x1, x2], [y, y], "-o", color=seg["color"], linewidth=3, markersize=8)
        mid_x = (x1 + x2) / 2
        ax.text(mid_x, y + 0.3, seg["label"], ha="center", fontsize=14)

    ax.set_xlim(0, width)
    ax.set_ylim(0, height + 0.5)
    ax.set_aspect("equal")
    ax.grid(True, alpha=0.3)
    ax.set_title("My Line Segments", fontsize=16)
    ax.set_axis_off()
    plt.tight_layout()
    plt.show()

    _segments.clear()
