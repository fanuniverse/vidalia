package image

import (
    "gopkg.in/gographics/imagick.v3/imagick"
)

type Image struct {
    path string
    ext string
    width uint
    height uint
    phash uint64
}

func NewImage(path string) (*Image, error) {
    ext, err := validImageExtension(path)

    if err != nil {
        return nil, err
    } else {
        return &Image{path: path, ext: ext}, nil
    }
}

func (image *Image) Process() error {
    imagick.Initialize()
    defer imagick.Terminate()

    wand := imagick.NewMagickWand()
    defer wand.Destroy()

    err := wand.ReadImage(image.path)
    if err != nil { return err }

    err = image.analyze(wand)
    if err != nil { return err }

    err = image.generateVersions(wand)
    return err
}
