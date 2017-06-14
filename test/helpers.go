package test

import (
    "os/exec"
    "testing"
    "github.com/stretchr/testify/require"
    "gopkg.in/gographics/imagick.v3/imagick"
    "github.com/buger/jsonparser"
)

func imageInfo(t *testing.T, path string) (width, height uint) {
    imagick.Initialize()
    defer imagick.Terminate()

    wand := imagick.NewMagickWand()
    defer wand.Destroy()

    err := wand.ReadImage(path)
    require.Nil(t, err)

    return wand.GetImageWidth(), wand.GetImageHeight()
}

func videoInfo(t *testing.T, path string) (width, height int64, codec string) {
    data, err := exec.Command("ffprobe", []string {
        "-v", "quiet", "-print_format", "json", "-show_streams", path,
    }...).Output()
    require.Nil(t, err)

    width, err = jsonparser.GetInt(data, "streams", "[0]", "width")
    require.Nil(t, err)

    height, err = jsonparser.GetInt(data, "streams", "[0]", "height")
    require.Nil(t, err)

    codec, err = jsonparser.GetString(data, "streams", "[0]", "codec_name")
    require.Nil(t, err)

    return width, height, codec
}
