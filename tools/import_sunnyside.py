#!/usr/bin/env python3
"""Import the Sunnyside World asset pack (Daniel Diggle, sold on itch.io) into the viewer.

The pack is a paid, non-redistributable asset pack, so nothing from it is kept in
this repository. Buy it, then run:

    python3 tools/import_sunnyside.py ~/Downloads/Sunnyside_World_ASSET_PACK_V2.1.zip

which writes viewer/assets/sunnyside/ (git-ignored). The viewer picks the pack up
automatically when that directory exists and falls back to the CC0 Generic RPG
Pack otherwise.

What is produced
  tileset.png          the 16px tileset as is (autotiles are addressed by id)
  chars/<anim>.png     one sheet per animation, 80x48 frames, rows = base, six
                       hair styles, tools; colours normalised for palette swaps
  chars.json           frame counts per animation
  props.png/props.json a packed atlas: trees, crops, animals, houses cut out of the
                       demo room, UI bars and bubbles, VFX strips
  windmill.png         the 9-frame windmill strip

Needs Pillow (pip install pillow).
"""
import io
import json
import os
import re
import sys
import zipfile

try:
    from PIL import Image
except ImportError:  # pragma: no cover
    sys.exit("Pillow is required: pip install pillow")

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "viewer", "assets", "sunnyside")
ASSETS = "Sunnyside_World_Assets/"
GM = "Sunnyside_World_Gamemaker/"

# animations kept for the viewer (name -> folder), 96x64 source frames cropped to 80x48
ANIMS = {"idle": "IDLE", "walk": "WALKING", "run": "RUN", "attack": "ATTACK", "death": "DEATH", "dig": "DIG",
         "doing": "DOING", "carry": "CARRY", "hurt": "HURT", "axe": "AXE", "watering": "WATERING", "mining": "MINING",
         "hammering": "HAMMERING", "jump": "JUMP"}
LAYERS = ["base", "bowlhair", "curlyhair", "longhair", "mophair", "shorthair", "spikeyhair", "tools"]
CROP = (16, 4, 96, 52)
# canonical colours: the strips carry near-duplicates (#e8ad7d/#e9ad7d/#e8ac7c ...), snap them for exact swaps
CANON = ["#e8ad7d", "#171424", "#374464", "#242b42", "#a51f33", "#fa717a", "#bb6d53", "#753d3a", "#3e2731", "#623530",
         "#945542", "#e07317", "#944526", "#f69345", "#ba4e2a", "#be4a2f", "#d36349", "#91402d", "#c58158", "#5d3b4a",
         "#e43b44", "#ed5259", "#a22633", "#ffffff", "#c0cbdc", "#8b9bb4", "#5a6988"]

# house prefabs: tile rectangles in the demo room (x0, y0, x1, y1), drawn from the building layers only
HOUSES = {"house_green": (21, 25, 25, 29), "house_red": (30, 26, 34, 30), "house_green2": (50, 25, 54, 29),
          "house_orange": (54, 24, 60, 29), "house_purple": (13, 26, 19, 32), "house_blue": (39, 24, 47, 30),
          "tower_green": (40, 32, 45, 38), "tower_orange": (55, 31, 61, 37)}
HOUSE_LAYERS = ["building", "walls", "decoration_02", "decoration_03"]

STRIPS = {  # name -> (path, frames)
    "tree_01": ("Elements/Plants/spr_deco_tree_01_strip4.png", 4), "tree_02": ("Elements/Plants/spr_deco_tree_02_strip4.png", 4),
    "mushroom_blue_01": ("Elements/Plants/spr_deco_mushroom_blue_01_strip4.png", 4), "mushroom_red_01": ("Elements/Plants/spr_deco_mushroom_red_01_strip4.png", 4),
    "chicken": ("Elements/Animals/spr_deco_chicken_01_strip4.png", 4), "cow": ("Elements/Animals/spr_deco_cow_strip4.png", 4),
    "pig": ("Elements/Animals/spr_deco_pig_01_strip4.png", 4), "sheep": ("Elements/Animals/spr_deco_sheep_01_strip4.png", 4),
    "duck": ("Elements/Animals/spr_deco_duck_01_strip4.png", 4), "bird": ("Elements/Animals/spr_deco_bird_01_strip4.png", 4),
    "fire": ("Elements/VFX/spr_deco_fire_02_strip4.png", 4), "glint": ("Elements/VFX/spr_deco_glint_01_strip6.png", 6),
    "coracle": ("Elements/Other/spr_deco_coracle_strip4.png", 4), "smoke": ("Elements/VFX/Chimney Smoke/chimneysmoke_01_strip30.png", 30),
}
SINGLES = {  # name -> path (first match wins; the pack moved a few files between versions)
    "campfire": "Elements/Other/spr_deco_campfire.png", "firepit": "Elements/Other/spr_deco_firepit.png",
    "well": "Elements/Other/spr_deco_well.png", "crate_01": "Elements/Other/spr_deco_crate_01.png", "crate_02": "Elements/Other/spr_deco_crate_02.png",
    "barrel_closed": "Elements/Other/spr_deco_barrel_closed.png", "barrel_open": "Elements/Other/spr_deco_barrel_open.png",
    "chest_closed": "Elements/Other/spr_deco_chest_01_closed.png", "chest_open": "Elements/Other/spr_deco_chest_01_open.png",
    "shadow": "Elements/Other/spr_deco_charactershadow.png", "rock": "Elements/Crops/rock.png", "wood": "Elements/Crops/wood.png",
    "egg": "Elements/Crops/egg.png", "fish": "Elements/Crops/fish.png", "seeds": "Elements/Crops/seeds_generic.png",
    "soil_00": "Elements/Crops/soil_00.png", "soil_01": "Elements/Crops/soil_01.png", "soil_03": "Elements/Crops/soil_03.png", "soil_04": "Elements/Crops/soil_04.png",
}
CROPS = ["wheat", "carrot", "pumpkin", "cabbage", "potato", "sunflower", "beetroot", "cauliflower", "kale", "parsnip", "radish"]
UI = ["expression_alerted", "expression_attack", "expression_chat", "expression_confused", "expression_love", "expression_stress", "expression_working",
      "happiness_01", "happiness_02", "happiness_03", "happiness_04", "indicator", "sword", "plant", "hammer", "axe", "pickaxe", "shovel", "basket", "water",
      "playercount", "sandtimer", "milk", "spr_deco_wool", "spr_deco_coin", "spr_deco_coins", "itemdisc_01", "label_left", "label_middle", "label_right",
      "selectbox_tl", "selectbox_tr", "selectbox_bl", "selectbox_br", "cursor_01", "confirm", "cancel"] + \
     [f"greenbar_0{i}" for i in range(7)] + [f"redbar_0{i}" for i in range(7)] + [f"bluebar_0{i}" for i in range(6)] + \
     [f"{k}_box_9slice_{p}" for k in ("dt", "lt", "w") for p in ("tl", "tc", "tr", "lc", "c", "rc", "bl", "bc", "br")]


class Pack:
    """Reads files from the zip or from an extracted directory."""

    def __init__(self, path):
        self.zip = zipfile.ZipFile(path) if os.path.isfile(path) else None
        self.root = None if self.zip else path
        if self.zip:
            self.names = [n for n in self.zip.namelist() if "__MACOSX" not in n]
        else:
            self.names = [os.path.relpath(os.path.join(d, f), path).replace(os.sep, "/") for d, _, fs in os.walk(path) for f in fs]
        self.prefix = ""
        for n in self.names:
            if n.endswith(ASSETS + "Tileset/spr_tileset_sunnysideworld_16px.png"):
                self.prefix = n[: -len(ASSETS + "Tileset/spr_tileset_sunnysideworld_16px.png")]
                break
        else:
            sys.exit("this does not look like the Sunnyside World pack (tileset not found)")
        self.gm_sprites = {}
        for n in self.names:
            m = re.search(re.escape(self.prefix + GM) + r"sprites/([^/]+)/([0-9a-f-]{36})\.png$", n)
            if m:
                self.gm_sprites.setdefault(m.group(1), []).append(n)

    def read(self, rel):
        n = self.prefix + rel
        if self.zip:
            return self.zip.read(n)
        with open(os.path.join(self.root, n), "rb") as f:
            return f.read()

    def image(self, rel):
        return Image.open(io.BytesIO(self.read(rel))).convert("RGBA")

    def find(self, suffix):
        for n in self.names:
            if n.endswith(suffix):
                return n[len(self.prefix):]
        return None

    def find_re(self, pattern):
        for n in self.names:
            if re.search(pattern, n):
                return n[len(self.prefix):]
        return None

    def sprite(self, name):
        """A GameMaker sprite's first frame (used for the many small UI/crop images)."""
        files = self.gm_sprites.get(name)
        if not files:
            return None
        return self.image(sorted(files)[0][len(self.prefix):])


def hex_rgb(h):
    return tuple(int(h[i:i + 2], 16) for i in (1, 3, 5))


def normalise(img, canon):
    """Snap near-duplicate colours to the canonical palette so the viewer can swap them exactly."""
    pal = [hex_rgb(h) for h in canon]
    px = img.load()
    cache = {}
    for y in range(img.height):
        for x in range(img.width):
            r, g, b, a = px[x, y]
            if not a:
                continue
            k = (r, g, b)
            if k in cache:
                px[x, y] = cache[k] + (a,)
                continue
            best, bd = k, 99
            for c in pal:
                d = abs(c[0] - r) + abs(c[1] - g) + abs(c[2] - b)
                if d < bd:
                    bd, best = d, c
            cache[k] = best if bd <= 12 else k
            px[x, y] = cache[k] + (a,)
    return img


def decode_tiles(data):
    """GameMaker TileCompressedData: negative n = repeat next value -n times, positive n = n literals."""
    out, i = [], 0
    while i < len(data):
        c = data[i]
        i += 1
        if c < 0:
            out.extend([data[i]] * -c)
            i += 1
        else:
            out.extend(data[i:i + c])
            i += c
    return out


def gm_json(text):
    return json.loads(re.sub(r",(\s*[}\]])", r"\1", text))


def room_layers(pack):
    room = gm_json(pack.read(GM + "rooms/Room1/Room1.yy").decode("utf-8"))
    layers = {}
    for L in room["layers"]:
        if L.get("resourceType") != "GMRTileLayer":
            continue
        t = L["tiles"]
        ids = decode_tiles(t["TileCompressedData"]) if "TileCompressedData" in t else list(t.get("TileSerialiseData", []))
        layers[L["name"]] = {"w": t["SerialiseWidth"], "h": t["SerialiseHeight"], "ids": ids}
    return layers


def draw_tile(ts, out, raw, x, y):
    if raw <= 0 or raw == 0x80000000:
        return
    idx = raw & 0x7FFFF
    if idx <= 0:
        return
    t = ts.crop(((idx % 64) * 16, (idx // 64) * 16, (idx % 64) * 16 + 16, (idx // 64) * 16 + 16))
    if raw & (1 << 28):
        t = t.transpose(Image.FLIP_LEFT_RIGHT)
    if raw & (1 << 29):
        t = t.transpose(Image.FLIP_TOP_BOTTOM)
    if raw & (1 << 30):
        t = t.transpose(Image.ROTATE_270)
    out.alpha_composite(t, (x, y))


def render_room(ts, layers, names, x0, y0, x1, y1):
    im = Image.new("RGBA", ((x1 - x0) * 16, (y1 - y0) * 16), (0, 0, 0, 0))
    for ln in names:
        L = layers.get(ln)
        if not L:
            continue
        for y in range(y0, y1):
            for x in range(x0, x1):
                if x < L["w"] and y < L["h"]:
                    draw_tile(ts, im, L["ids"][y * L["w"] + x], (x - x0) * 16, (y - y0) * 16)
    return im


class Atlas:
    """A simple shelf packer."""

    def __init__(self, width=1024):
        self.width, self.x, self.y, self.row_h = width, 0, 0, 0
        self.items, self.entries = [], {}

    def add(self, name, img, frames=1):
        w, h = img.size
        if self.x + w > self.width:
            self.x, self.y, self.row_h = 0, self.y + self.row_h + 1, 0
        self.items.append((img, self.x, self.y))
        self.entries[name] = {"x": self.x, "y": self.y, "w": w, "h": h, "frames": frames}
        self.x += w + 1
        self.row_h = max(self.row_h, h)

    def save(self, png, js):
        out = Image.new("RGBA", (self.width, self.y + self.row_h), (0, 0, 0, 0))
        for img, x, y in self.items:
            out.alpha_composite(img, (x, y))
        out.save(png)
        with open(js, "w") as f:
            json.dump(self.entries, f, separators=(",", ":"))
        return out.size


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    pack = Pack(sys.argv[1])
    out = os.path.abspath(sys.argv[2] if len(sys.argv) > 2 else OUT)
    os.makedirs(os.path.join(out, "chars"), exist_ok=True)

    ts = pack.image(ASSETS + "Tileset/spr_tileset_sunnysideworld_16px.png")
    ts.save(os.path.join(out, "tileset.png"))
    print("tileset", ts.size)

    # characters: one sheet per animation, rows = layers
    canon = CANON
    frames = {}
    for anim, folder in ANIMS.items():
        strips = []
        for layer in LAYERS:
            rel = pack.find_re(rf"Characters/Human/{folder}/{layer}_[a-z]+_strip\d+\.png$")
            strips.append(pack.image(rel) if rel else None)
        n = max(s.width // 96 for s in strips if s)
        sheet = Image.new("RGBA", (n * 80, len(LAYERS) * 48), (0, 0, 0, 0))
        for r, s in enumerate(strips):
            if s is None:
                continue
            for k in range(min(n, s.width // 96)):
                fr = s.crop((k * 96 + CROP[0], CROP[1], k * 96 + CROP[2], CROP[3]))
                sheet.alpha_composite(fr, (k * 80, r * 48))
        normalise(sheet, canon).save(os.path.join(out, "chars", anim + ".png"))
        frames[anim] = n
        print("chars", anim, n, "frames")
    with open(os.path.join(out, "chars.json"), "w") as f:
        json.dump({"frames": frames, "layers": LAYERS, "w": 80, "h": 48, "anchor": [0.406, 0.74]}, f)

    # props atlas
    atlas = Atlas()
    for name, (rel, n) in STRIPS.items():
        p = pack.find(rel.split("/")[-1])
        if p:
            atlas.add(name, pack.image(p), n)
        else:
            print("  missing strip", rel)
    for name, rel in SINGLES.items():
        p = pack.find(rel.split("/")[-1])
        img = pack.image(p) if p else pack.sprite(rel.split("/")[-1].replace(".png", ""))
        if img is not None:
            atlas.add(name, img)
        else:
            print("  missing", rel)
    for crop in CROPS:
        for k in range(6):
            p = pack.find(f"Elements/Crops/{crop}_0{k}.png")
            if p:
                atlas.add(f"{crop}_{k}", pack.image(p))
    for name in UI:
        img = pack.sprite(name)
        if img is not None:
            atlas.add(name, img)
        else:
            print("  missing ui", name)
    layers = room_layers(pack)
    for name, (x0, y0, x1, y1) in HOUSES.items():
        atlas.add(name, render_room(ts, layers, HOUSE_LAYERS, x0, y0, x1, y1))
    # cloud blobs cut from the demo room's cloud layer
    L = layers.get("clouds_01")
    if L:
        w, h, ids = L["w"], L["h"], L["ids"]
        seen = [False] * (w * h)
        blobs = []
        for i in range(w * h):
            if (ids[i] & 0x7FFFF) and not seen[i]:
                st, cells = [i], []
                seen[i] = True
                while st:
                    j = st.pop()
                    cells.append(j)
                    for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                        nx, ny = j % w + dx, j // w + dy
                        if 0 <= nx < w and 0 <= ny < h and (ids[ny * w + nx] & 0x7FFFF) and not seen[ny * w + nx]:
                            seen[ny * w + nx] = True
                            st.append(ny * w + nx)
                xs, ys = [c % w for c in cells], [c // w for c in cells]
                blobs.append((min(xs), min(ys), max(xs) + 1, max(ys) + 1, len(cells)))
        blobs = [b for b in blobs if 4 <= b[4] <= 40 and b[2] - b[0] <= 8 and b[3] - b[1] <= 6]
        blobs.sort(key=lambda b: -b[4])
        for k, (x0, y0, x1, y1, _) in enumerate(blobs[:6]):
            atlas.add(f"cloud_{k}", render_room(ts, layers, ["clouds_01"], x0, y0, x1, y1))
    size = atlas.save(os.path.join(out, "props.png"), os.path.join(out, "props.json"))
    print("props atlas", size, len(atlas.entries), "entries")

    wm = pack.find("spr_deco_windmill_strip9.png")
    if wm:
        pack.image(wm).save(os.path.join(out, "windmill.png"))
    with open(os.path.join(out, "README.txt"), "w") as f:
        f.write("Generated by tools/import_sunnyside.py from the Sunnyside World asset pack.\n"
                "These files are licensed to the buyer only: do not commit or redistribute them.\n")
    print("done ->", out)


if __name__ == "__main__":
    main()
