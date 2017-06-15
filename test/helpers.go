package test

import (
    "os"
    "os/exec"
    "time"
    "path"
    "testing"
    "encoding/json"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
    "gopkg.in/gographics/imagick.v3/imagick"
    "github.com/buger/jsonparser"
    "github.com/streadway/amqp"
    "vidalia/runner"
    "vidalia/image"
    "vidalia/config"
)

func processOverAmqp(t *testing.T, file, id string) (img image.Image) {
    conn, ch := runner.ObtainChannelConnection()
    go runner.RunService(ch)

    body, _ := json.Marshal(map[string]string{"id": id, "file": file})
    err := ch.Publish(
        "", "image.process", false, false, amqp.Publishing {
            DeliveryMode:    amqp.Transient,
            Timestamp:       time.Now(),
            ContentType:     "application/json",
            Body:            body,
       })
    require.Nil(t, err)

    processed, err := ch.Consume(
        "image.processed", "", false, false, false, false, nil)
    require.Nil(t, err)

    for m := range processed {
        err := json.Unmarshal(m.Body, &img)
        require.Nil(t, err)

        m.Ack(true)
        break
    }
    ch.Close()
    conn.Close()

    return img
}

func assertStored(t *testing.T, file, id string, img image.Image) {
    cached := path.Join(config.CacheDir, file)
    target := path.Join(config.StorageDir, id, "source." + img.Ext)

    assert.Equal(t, target, img.Path,
        "image.Path should be updated with the storage location.")

    if _, err := os.Stat(cached); os.IsExist(err) {
        assert.Fail(t, "Cached file should be destroyed (moved)")
    }
    if _, err := os.Stat(target); os.IsNotExist(err) {
        assert.Fail(t, "Target file should exist")
    }
}

func cleanUpStored(file, id string, img image.Image) {
    cached := path.Join(config.CacheDir, file)
    targetDir := path.Join(config.StorageDir, id)
    target := path.Join(targetDir, "source." + img.Ext)

    os.Rename(target, cached)
    os.RemoveAll(targetDir)
}

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
