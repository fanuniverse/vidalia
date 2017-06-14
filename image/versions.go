package image

import (
    "errors"
    "gopkg.in/gographics/imagick.v3/imagick"
)

func (image *Image) generateVersions(wand *imagick.MagickWand) (err error) {
    if image.Width == 0 || image.Height == 0 {
        return errors.New("Image struct doesn't have width and height set.")
    }

    if image.Ext == "gif" {
        return image.generateAnimatedVersions()
    } else {
        return image.generateRasterVersions(wand)
    }
}
