package image

import (
    "path"
    "gopkg.in/gographics/imagick.v3/imagick"
    "vidalia/config"
)

type Image struct {
    // Storage
    Path   string `json:"path"`
    Ext    string `json:"ext"`
    Id     string `json:"id"`
    // Metadata
    Width  uint   `json:"width"`
    Height uint   `json:"height"`
    Phash  uint64 `json:"phash"`
}

func NewImage(cachedFile, id string) (*Image, error) {
    path := path.Join(config.CacheDir, cachedFile)
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
