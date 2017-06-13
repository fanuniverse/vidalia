package image

import (
    "math"
    "errors"
    "gopkg.in/gographics/imagick.v3/imagick"
)

var Versions = map[string]uint {
    "thumbnail": 300,
    "preview": 1280,
}

var AnimatedVersions = map[string]string {
    "mp4": "rendered.mp4",
    "webm": "rendered.webm",
    "poster": "poster.jpg",
}

func (image *Image) generateVersions(wand *imagick.MagickWand) (err error) {
    if image.width == 0 || image.height == 0 {
        return errors.New("Image struct doesn't have width and height set.")
    }

    ratio := float64(image.width) / float64(image.height)

    for version, width := range Versions {
        if width < image.width {
            err = image.createVersion(wand, version, width, ratio)
        } else {
            err = image.linkVersionToImage(version)
        }

        if err != nil { break }
    }

    return err
}

func (image *Image) createVersion(wand *imagick.MagickWand,
        version string, width uint, ratio float64) (err error) {
    path := image.versionStoragePath(version)
    height := uint(math.Floor(float64(width) * ratio))

    versionWand := wand.Clone()
    defer versionWand.Destroy()

    err = versionWand.ResizeImage(width, height, imagick.FILTER_UNDEFINED)
    if err != nil { return err }

    err = versionWand.SetImageCompressionQuality(95)
    if err != nil { return err }

    err = versionWand.WriteImage(path)
    return err
}
