from pathlib import Path

from tga_compressor.compressor import comress_all

if __name__ == "__main__":
    src = Path("..", "koldun", "resources", "tiles", "tga")
    dest = Path("..", "koldun", "resources", "tiles", "compressed")
    comress_all(src, dest)
