package test

import (
    "os"
    "path"
    "testing"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
    "vidalia/image"
    "vidalia/config"
)

func TestAnimatedImage(t *testing.T) {
    cached := path.Join(config.CacheDir, "animated.gif")
    targetDir := path.Join(config.StorageDir, "24")
    target := path.Join(targetDir, "source.gif")

    img, err := image.NewImage(cached, 24)

    require.Nil(t, err)
    assert.Equal(t, target, img.Path,
        "image.Path should be updated with the storage location.")

    if _, err := os.Stat(cached); os.IsExist(err) {
        assert.Fail(t, "Cached file should be destroyed (moved)")
    }
    if _, err := os.Stat(target); os.IsNotExist(err) {
        assert.Fail(t, "Target file should exist")
    }

    err = img.Process()
    require.Nil(t, err)

    /* Reset test image location for subsequent tests */
    defer func() {
        os.Rename(target, cached)
        os.RemoveAll(targetDir)
    }()

    /* Analysis test */
    assert.Equal(t, uint(500), img.Width)
    assert.Equal(t, uint(280), img.Height)
    assert.Equal(t, uint64(8330785626518358964), img.Phash)

    /* Version test */
    webmW, webmH, codec := videoInfo(t,
        path.Join(config.StorageDir, "24/rendered.webm"))
    assert.Equal(t, "vp9", codec)
    assert.Equal(t, int64(500), webmW)
    assert.Equal(t, int64(280), webmH)

    mp4W, mp4H, codec := videoInfo(t,
        path.Join(config.StorageDir, "24/rendered.mp4"))
    assert.Equal(t, "h264", codec)
    assert.Equal(t, int64(500), mp4W)
    assert.Equal(t, int64(280), mp4H)

    posterW, posterH := imageInfo(t,
        path.Join(config.StorageDir, "24/poster.jpg"))
    assert.Equal(t, uint(500), posterW)
    assert.Equal(t, uint(280), posterH)
}
