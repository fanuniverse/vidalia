package image

import (
    "os"
    "fmt"
    "math"
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
    iWidth := wand.GetImageWidth()
    iHeight := wand.GetImageHeight()
    ratio := float64(iWidth) / float64(iHeight)

    for version, vWidth := range Versions {
        // TODO: parse the MIME type
        vPath := fmt.Sprintf("test/images/%s.jpg", version)

        if vWidth < iWidth {
            vWand := wand.Clone()
            err = image.createVersion(vWand, vWidth, ratio, vPath)
            vWand.Destroy()
        } else {
            err = image.linkToImage(vPath)
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
