import logging
from pathlib import Path

import numpy as np
import numpy.typing as tp
from PIL import Image


def _compress(path: Path) -> tp.ArrayLike:
    image = Image.open(path)
    data = image.getdata()
    data = [any(pixel) for pixel in data]

    return np.packbits(data, bitorder="little")


def compress(src: Path, dest: Path) -> None:
    bits = _compress(src)
    with open(dest, mode="wb") as f:
        f.write(bits)


def comress_all(src: Path, dest: Path):
    for file in src.iterdir():
        dest_file = dest / Path(f"{file.stem}.bin")
        compress(file, dest_file)
        logging.info(f"{file} converted to {dest_file}")
