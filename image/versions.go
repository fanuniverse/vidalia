package image

import (
    "os"
    "fmt"
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
        path := fmt.Sprintf("test/images/%s.%s", version, image.ext)

        if width < image.width {
            versionWand := wand.Clone()
            err = image.createVersion(versionWand, width, ratio, path)
            versionWand.Destroy()
        } else {
            err = image.linkToImage(path)
        }

        if err != nil { break }
    }

    return err
}

func (image *Image) createVersion(wand *imagick.MagickWand,
        width uint, ratio float64, path string) error {
    height := uint(math.Floor(float64(width) * ratio))

    err := wand.ResizeImage(width, height, imagick.FILTER_UNDEFINED)
    if err != nil { return err }

    err = wand.SetImageCompressionQuality(95)
    if err != nil { return err }

    err = wand.WriteImage(path)
    return err
}

func (image *Image) linkToImage(path string) error {
    return os.Link(image.path, path)
}
