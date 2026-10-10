import os
from PIL import Image, ImageDraw, ImageFont

def draw_logo(size: int) -> Image.Image:
    # Render at 4x resolution for super-sampled anti-aliasing
    scale = 4
    canvas_size = size * scale
    img = Image.new("RGBA", (canvas_size, canvas_size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    pad = canvas_size * 0.06
    r = canvas_size * 0.22

    # Squircle dark background with subtle border
    draw.rounded_rectangle(
        [pad, pad, canvas_size - pad, canvas_size - pad],
        radius=int(r),
        fill=(11, 15, 25, 255),       # Deep dark slate/graphite
        outline=(30, 41, 59, 255),    # 1px border #1e293b
        width=max(1, int(canvas_size * 0.02))
    )

    # Kite geometry
    cx = canvas_size / 2.0
    cy = canvas_size * 0.46  # Spar center slightly above midpoint for true kite proportion
    
    top = (cx, canvas_size * 0.16)
    left = (canvas_size * 0.18, cy)
    right = (canvas_size * 0.82, cy)
    bottom = (cx, canvas_size * 0.84)
    center = (cx, cy)

    # Four architectural facets with crisp contrast
    # Top-Left: Bright sky
    draw.polygon([top, left, center], fill=(14, 165, 233, 255))
    # Top-Right: Electric cyan highlight
    draw.polygon([top, right, center], fill=(56, 189, 248, 255))
    # Bottom-Left: Deep cobalt/slate
    draw.polygon([bottom, left, center], fill=(3, 105, 161, 255))
    # Bottom-Right: Rich sapphire
    draw.polygon([bottom, right, center], fill=(2, 132, 199, 255))

    # Architectural spar lines (thin crisp wireframe grid)
    line_w = max(1, int(canvas_size * 0.015))
    draw.line([top, bottom], fill=(240, 249, 255, 140), width=line_w)
    draw.line([left, right], fill=(240, 249, 255, 140), width=line_w)

    # Outer perimeter stroke for sharp definition
    draw.line([top, right, bottom, left, top], fill=(186, 230, 253, 200), width=max(1, int(canvas_size * 0.02)))

    # Inscribed Asymptotic "0" (Slashed Zero) at center
    # Outer ring
    rw = canvas_size * 0.15
    rh = canvas_size * 0.19
    ring_box = [cx - rw, cy - rh, cx + rw, cy + rh]

    # Dark background plate behind zero for ultra-high legibility
    bg_rw = rw + canvas_size * 0.03
    bg_rh = rh + canvas_size * 0.03
    draw.ellipse([cx - bg_rw, cy - bg_rh, cx + bg_rw, cy + bg_rh], fill=(11, 15, 25, 230))

    # Ring stroke
    stroke_w = max(2, int(canvas_size * 0.045))
    draw.ellipse(ring_box, outline=(255, 255, 255, 255), width=stroke_w)

    # Slashed zero diagonal bar: connects top-right of inner ring to bottom-left
    slash_dx = rw * 0.72
    slash_dy = rh * 0.72
    draw.line(
        [(cx + slash_dx, cy - slash_dy), (cx - slash_dx, cy + slash_dy)],
        fill=(255, 255, 255, 255),
        width=max(1, int(stroke_w * 0.75))
    )

    # Downscale with high-quality Lanczos resampling
    final_img = img.resize((size, size), Image.Resampling.LANCZOS)
    return final_img

def generate_svg() -> str:
    return '''<svg width="64" height="64" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
  <rect x="2" y="2" width="60" height="60" rx="14" fill="#0B0F19" stroke="#1E293B" stroke-width="1.5"/>
  <!-- Kite Facets -->
  <polygon points="32,10 11.5,29.5 32,29.5" fill="#0EA5E9"/>
  <polygon points="32,10 52.5,29.5 32,29.5" fill="#38BDF8"/>
  <polygon points="32,54 11.5,29.5 32,29.5" fill="#0369A1"/>
  <polygon points="32,54 52.5,29.5 32,29.5" fill="#0284C7"/>
  <!-- Perimeter & Internal Spars -->
  <polygon points="32,10 52.5,29.5 32,54 11.5,29.5" stroke="#BAE6FD" stroke-width="1" stroke-linejoin="round" fill="none"/>
  <line x1="32" y1="10" x2="32" y2="54" stroke="#F0F9FF" stroke-opacity="0.5" stroke-width="1"/>
  <line x1="11.5" y1="29.5" x2="52.5" y2="29.5" stroke="#F0F9FF" stroke-opacity="0.5" stroke-width="1"/>
  <!-- Central Slashed Zero Shield -->
  <ellipse cx="32" cy="29.5" rx="11" ry="13.5" fill="#0B0F19" fill-opacity="0.9"/>
  <ellipse cx="32" cy="29.5" rx="9" ry="11.5" stroke="#FFFFFF" stroke-width="2.5" fill="none"/>
  <line x1="38" y1="21.5" x2="26" y2="37.5" stroke="#FFFFFF" stroke-width="2" stroke-linecap="round"/>
</svg>
'''

def main():
    target_dir = os.path.join("extension", "public", "icons")
    os.makedirs(target_dir, exist_ok=True)
    
    # Generate PNG icons
    for s in [16, 48, 128]:
        icon = draw_logo(s)
        path = os.path.join(target_dir, f"icon-{s}.png")
        icon.save(path, "PNG")
        print(f"Generated {path} ({s}x{s})")

    # Generate SVG logo
    svg_path = os.path.join(target_dir, "kite0-logo.svg")
    with open(svg_path, "w", encoding="utf-8") as f:
        f.write(generate_svg())
    print(f"Generated {svg_path}")

if __name__ == "__main__":
    main()
