package image

import (
    "gopkg.in/gographics/imagick.v3/imagick"
    "github.com/kavu/go-phash"
)

func (image *Image) analyze(wand *imagick.MagickWand) error {
    image.width = wand.GetImageWidth()
    image.height = wand.GetImageHeight()

    imageHash, err := phash.ImageHashDCT(image.path)
    image.phash = imageHash

    return err
}
