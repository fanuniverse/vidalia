package image

import (
    "gopkg.in/gographics/imagick.v3/imagick"
    "github.com/kavu/go-phash"
)

func (image *Image) analyze(wand *imagick.MagickWand) error {
    image.Width = wand.GetImageWidth()
    image.Height = wand.GetImageHeight()

    imageHash, err := phash.ImageHashDCT(image.Path)
    image.Phash = imageHash

    return err
}
