import os
from PIL import Image, ImageDraw

def generate_kite_icon(size):
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Rounded background
    padding = size * 0.08
    r = size * 0.22
    draw.rounded_rectangle(
        [padding, padding, size - padding, size - padding],
        radius=r,
        fill=(15, 23, 42, 255),  # Slate 900
        outline=(56, 189, 248, 255), # Sky 400
        width=max(1, int(size * 0.04))
    )

    # Kite diamond polygon
    center_x = size / 2.0
    center_y = size / 2.0
    top = (center_x, size * 0.22)
    bottom = (center_x, size * 0.78)
    left = (size * 0.25, center_y)
    right = (size * 0.75, center_y)

    # Gradient-like facets
    draw.polygon([top, right, (center_x, center_y)], fill=(56, 189, 248, 255))
    draw.polygon([top, left, (center_x, center_y)], fill=(14, 165, 233, 255))
    draw.polygon([bottom, left, (center_x, center_y)], fill=(2, 132, 199, 255))
    draw.polygon([bottom, right, (center_x, center_y)], fill=(3, 105, 161, 255))

    return img

def main():
    os.makedirs("extension/public/icons", exist_ok=True)
    for s in [16, 48, 128]:
        icon = generate_kite_icon(s)
        path = f"extension/public/icons/icon-{s}.png"
        icon.save(path, "PNG")
        print(f"Generated {path}")

if __name__ == "__main__":
    main()
