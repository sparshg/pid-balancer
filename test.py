from PIL import Image
img = Image.open("vingette.png")
pixels = img.load()
for y in range(img.height):
    for x in range(img.width):
        pixels[x, y] = (0, 0, 0, 255 - pixels[x, y][0])
img.save("vingette2.png")
