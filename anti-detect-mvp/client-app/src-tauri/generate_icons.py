from PIL import Image
import os

icons_dir = os.path.join(os.path.dirname(__file__), 'icons')
os.makedirs(icons_dir, exist_ok=True)

def make_png(path, size, color=(255,140,0,255)):
    img = Image.new('RGBA', size, (0,0,0,0))
    from PIL import ImageDraw
    draw = ImageDraw.Draw(img)
    cx, cy = size[0]//2, size[1]//2
    r = min(size)//2 - 4
    draw.ellipse((cx-r, cy-r, cx+r, cy+r), fill=color)
    img.save(path, format='PNG')
    print('Wrote', path)

p32 = os.path.join(icons_dir, '32x32.png')
p128 = os.path.join(icons_dir, '128x128.png')
p128_2x = os.path.join(icons_dir, '128x128@2x.png')
pico = os.path.join(icons_dir, 'icon.ico')

if not os.path.exists(p32):
    make_png(p32, (32,32))
else:
    print('Exists', p32)

make_png(p128, (128,128))
make_png(p128_2x, (256,256))

sizes = [(32,32),(128,128),(256,256)]
img_for_ico = Image.new('RGBA', (256,256), (0,0,0,0))
from PIL import ImageDraw
draw = ImageDraw.Draw(img_for_ico)
draw.ellipse((16,16,240,240), fill=(255,140,0,255))
img_for_ico.save(pico, format='ICO', sizes=sizes)
print('Wrote', pico)
