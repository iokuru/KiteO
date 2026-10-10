import os
from PIL import Image, ImageDraw

def draw_clean_logo(size: int) -> Image.Image:
    scale = 4
    canvas_size = size * scale
    img = Image.new("RGBA", (canvas_size, canvas_size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    pad = canvas_size * 0.05
    cx = canvas_size / 2.0
    cy = canvas_size / 2.0
    radius = (canvas_size - 2 * pad) / 2.0

    # Solid circular badge (clean royal blue #2563eb)
    draw.ellipse(
        [cx - radius, cy - radius, cx + radius, cy + radius],
        fill=(37, 99, 235, 255)
    )

    # Clean geometric kite wireframe in pure white
    top = (cx, cy - radius * 0.65)
    bottom = (cx, cy + radius * 0.70)
    left = (cx - radius * 0.60, cy - radius * 0.05)
    right = (cx + radius * 0.60, cy - radius * 0.05)
    center = (cx, cy - radius * 0.05)

    line_w = max(1, int(canvas_size * 0.045))
    thin_w = max(1, int(canvas_size * 0.025))

    # Internal spars
    draw.line([top, bottom], fill=(255, 255, 255, 180), width=thin_w)
    draw.line([left, right], fill=(255, 255, 255, 180), width=thin_w)

    # Perimeter outline
    draw.line([top, right, bottom, left, top], fill=(255, 255, 255, 255), width=line_w)

    final_img = img.resize((size, size), Image.Resampling.LANCZOS)
    return final_img

def main():
    target_dir = os.path.join("extension", "public", "icons")
    os.makedirs(target_dir, exist_ok=True)
    
    for s in [16, 48, 128]:
        icon = draw_clean_logo(s)
        path = os.path.join(target_dir, f"icon-{s}.png")
        icon.save(path, "PNG")
        print(f"Generated {path} ({s}x{s})")

if __name__ == "__main__":
    main()
