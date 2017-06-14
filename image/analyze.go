package image

import (
    "gopkg.in/gographics/imagick.v3/imagick"
    "github.com/kavu/go-phash"
)

func (image *Image) analyze(wand *imagick.MagickWand) error {
    if image.Ext == "gif" {
        w, h, _, _, err := wand.GetImagePage()
        if err != nil { return err }

        image.Width = w
        image.Height = h
    } else {
        image.Width = wand.GetImageWidth()
        image.Height = wand.GetImageHeight()
    }

    imageHash, err := phash.ImageHashDCT(image.Path)
    image.Phash = imageHash

    return err
}
