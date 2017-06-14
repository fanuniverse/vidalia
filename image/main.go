package image

import (
    "gopkg.in/gographics/imagick.v3/imagick"
)

type Image struct {
    // Storage
    Path string
    Ext string
    Id uint
    // Metadata
    Width uint
    Height uint
    Phash uint64
}

func NewImage(path string, id uint) (*Image, error) {
    ext, err := validImageExtension(path)
    if err != nil { return nil, err }

    image := Image{Path: path, Ext: ext, Id: id}

    err = image.moveCachedToStorage()
    if err != nil { return nil, err }

    return &image, nil
}

func (image *Image) Process() error {
    imagick.Initialize()
    defer imagick.Terminate()

    wand := imagick.NewMagickWand()
    defer wand.Destroy()

    err := wand.ReadImage(image.Path)
    if err != nil { return err }

    err = image.analyze(wand)
    if err != nil { return err }

    err = image.generateVersions(wand)
    return err
}
